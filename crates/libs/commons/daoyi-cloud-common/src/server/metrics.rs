use salvo::prelude::*;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

pub static REQUEST_COUNT: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));
pub static ACTIVE_REQUESTS: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));
pub static ERROR_4XX: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));
pub static ERROR_5XX: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));

#[derive(Clone)]
pub struct MetricsMiddleware;

impl MetricsMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[handler]
impl MetricsMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
        ACTIVE_REQUESTS.fetch_add(1, Ordering::Relaxed);

        ctrl.call_next(req, depot, res).await;

        ACTIVE_REQUESTS.fetch_sub(1, Ordering::Relaxed);
        let status = res.status_code.unwrap_or(StatusCode::OK);
        let code: u16 = status.into();
        if (400..500).contains(&code) {
            ERROR_4XX.fetch_add(1, Ordering::Relaxed);
        } else if code >= 500 {
            ERROR_5XX.fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[endpoint]
pub async fn report(res: &mut Response) {
    let total = REQUEST_COUNT.load(Ordering::Relaxed);
    let active = ACTIVE_REQUESTS.load(Ordering::Relaxed);
    let err_4xx = ERROR_4XX.load(Ordering::Relaxed);
    let err_5xx = ERROR_5XX.load(Ordering::Relaxed);

    let body = format!(
        "# HELP http_requests_total Total HTTP requests\n\
         # TYPE http_requests_total counter\n\
         http_requests_total {total}\n\n\
         # HELP http_requests_active Currently active requests\n\
         # TYPE http_requests_active gauge\n\
         http_requests_active {active}\n\n\
         # HELP http_requests_errors_total Total HTTP errors by status class\n\
         # TYPE http_requests_errors_total counter\n\
         http_requests_errors_total{{code=\"4xx\"}} {err_4xx}\n\
         http_requests_errors_total{{code=\"5xx\"}} {err_5xx}\n"
    );

    use salvo::http::header::CONTENT_TYPE;
    res.headers_mut()
        .insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap());
    res.render(body);
}
