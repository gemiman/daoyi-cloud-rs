use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 超时时间戳（秒）
type ExpiresAt = u64;

/// 简单内存缓存（线程安全）
pub struct SimpleCache {
    inner: Mutex<HashMap<String, (String, ExpiresAt)>>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl SimpleCache {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let now = now_secs();
        let map = self.inner.lock().unwrap();
        if let Some((value, exp)) = map.get(key) {
            if *exp > now {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return Some(value.clone());
            }
        }
        self.misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub fn set(&self, key: String, value: String, ttl_secs: u64) {
        let exp = now_secs() + ttl_secs;
        let mut map = self.inner.lock().unwrap();
        map.insert(key, (value, exp));
    }

    pub fn invalidate(&self, key: &str) {
        let mut map = self.inner.lock().unwrap();
        map.remove(key);
    }

    pub fn stats(&self) -> (u64, u64) {
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
        )
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// 用户缓存实例（5 分钟过期）
pub static USER_CACHE: LazyLock<SimpleCache> = LazyLock::new(SimpleCache::new);
