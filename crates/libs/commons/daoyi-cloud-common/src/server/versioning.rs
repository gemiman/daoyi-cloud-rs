/// API 版本管理帮助函数
///
/// 在路由前使用 `/v1` 前缀：
///
/// ```ignore
/// Router::with_path(api_v1("users")).get(list_users)
/// ```
pub fn api_v1(path: &str) -> String {
    format!("/v1/{}", path.trim_start_matches('/'))
}
