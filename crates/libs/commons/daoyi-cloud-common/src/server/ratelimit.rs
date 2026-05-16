use salvo::prelude::*;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 默认限流：每分钟 1000 次请求
const DEFAULT_MAX_REQUESTS: u64 = 1000;
const DEFAULT_WINDOW_SECS: i64 = 60;

/// 全局限流器（固定窗口）
struct FixedWindowRateLimiter {
    max_requests: u64,
    window_secs: i64,
    counter: AtomicU64,
    window_start: AtomicI64,
}

impl FixedWindowRateLimiter {
    const fn new(max_requests: u64, window_secs: i64) -> Self {
        Self {
            max_requests,
            window_secs,
            counter: AtomicU64::new(0),
            window_start: AtomicI64::new(0),
        }
    }

    fn check(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mut ws = self.window_start.load(Ordering::Relaxed);

        // 当前窗口已过期，重置
        if now - ws >= self.window_secs {
            // CAS 竞争：只有第一个线程能重置
            let result =
                self.window_start
                    .compare_exchange(ws, now, Ordering::SeqCst, Ordering::Relaxed);
            if result.is_ok() {
                self.counter.store(1, Ordering::Relaxed);
                return true;
            }
            // 其他线程：重新读取 window_start
            ws = self.window_start.load(Ordering::Relaxed);
            // 如果新窗口仍然过期（极少发生的时间边界情况），等待下一次
            if now - ws >= self.window_secs {
                return false;
            }
        }

        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        count < self.max_requests
    }
}

static RATE_LIMITER: LazyLock<FixedWindowRateLimiter> = LazyLock::new(|| {
    let max = std::env::var("RATE_LIMIT_MAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MAX_REQUESTS);
    let window = std::env::var("RATE_LIMIT_WINDOW_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_WINDOW_SECS);
    tracing::info!("Rate limiter initialized: max={}/{}s", max, window);
    FixedWindowRateLimiter::new(max, window)
});

/// 限流中间件
#[derive(Clone)]
pub struct RateLimitMiddleware;

impl RateLimitMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[handler]
impl RateLimitMiddleware {
    async fn handle(
        &self,
        _req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if RATE_LIMITER.check() {
            ctrl.call_next(_req, _depot, res).await;
        } else {
            res.status_code(StatusCode::TOO_MANY_REQUESTS);
            res.render("Rate limit exceeded. Please try again later.");
            ctrl.skip_rest();
        }
    }
}
