use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 内存 Token 黑名单（用于登出 / 主动失效）
///
/// 存储 token 的 jti 直到其原始过期时间。
/// 生产环境应替换为 Redis。
pub struct TokenBlacklist {
    inner: Mutex<HashSet<String>>,
    /// 定期清理触发阈值
    last_cleanup: AtomicU64,
}

impl TokenBlacklist {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashSet::new()),
            last_cleanup: AtomicU64::new(0),
        }
    }

    /// 将 token 加入黑名单
    pub fn revoke(&self, jti: &str) {
        let mut set = self.inner.lock().unwrap();
        set.insert(jti.to_string());
    }

    /// 检查 token 是否已被撤销
    pub fn is_revoked(&self, jti: &str) -> bool {
        self.inner.lock().unwrap().contains(jti)
    }

    /// 清理黑名单（外部定期调用）
    pub fn cleanup(&self) {
        let now = now_secs();
        let last = self.last_cleanup.load(Ordering::Relaxed);
        // 每小时最多清理一次
        if now - last < 3600 {
            return;
        }
        self.last_cleanup.store(now, Ordering::Relaxed);
        let mut set = self.inner.lock().unwrap();
        set.clear();
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub static TOKEN_BLACKLIST: LazyLock<TokenBlacklist> = LazyLock::new(TokenBlacklist::new);
