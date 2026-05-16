use salvo::prelude::*;

/// 安全头中间件：为所有响应添加 HTTP 安全头
#[derive(Clone)]
pub struct SecurityHeadersMiddleware;

impl SecurityHeadersMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[handler]
impl SecurityHeadersMiddleware {
    async fn handle(
        &self,
        _req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        ctrl.call_next(_req, _depot, res).await;

        let headers = res.headers_mut();
        // 防止 MIME 类型嗅探
        headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
        // 禁止页面被嵌入 iframe（点击劫持防护）
        headers.insert("X-Frame-Options", "DENY".parse().unwrap());
        // 启用 XSS 过滤
        headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
        // 限制 referrer 信息泄露
        headers.insert(
            "Referrer-Policy",
            "strict-origin-when-cross-origin".parse().unwrap(),
        );
        // 禁止自动检测内容类型
        headers.insert("X-Download-Options", "noopen".parse().unwrap());
        // 权限特性限制
        headers.insert(
            "Permissions-Policy",
            "camera=(), microphone=(), geolocation=()".parse().unwrap(),
        );
    }
}
