use std::time::Instant;

/// 测量异步操作的执行时长并记录 trace 日志。
///
/// # 示例
///
/// ```ignore
/// let result = timing!("查询用户列表", {
///     SysUser::find().all(db).await?
/// });
/// ```
#[macro_export]
macro_rules! timing {
    ($label:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        if elapsed.as_millis() > 100 {
            tracing::warn!(
                target: "slow_query",
                elapsed_ms = elapsed.as_millis(),
                label = $label,
                "SLOW QUERY"
            );
        } else {
            tracing::debug!(
                target: "query",
                elapsed_us = elapsed.as_micros(),
                label = $label,
            );
        }
        result
    }};
}

/// 手动计时工具函数
pub async fn timed<T, F, Fut>(label: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let start = Instant::now();
    let result = f().await;
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 100 {
        tracing::warn!(target: "slow_query", elapsed_ms = elapsed.as_millis(), label = label, "SLOW QUERY");
    } else {
        tracing::debug!(target: "query", elapsed_us = elapsed.as_micros(), label = label);
    }
    result
}
