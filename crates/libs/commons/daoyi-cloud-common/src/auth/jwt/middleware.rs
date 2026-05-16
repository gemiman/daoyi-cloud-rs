use crate::auth::jwt::{Principal, default_jwt};
use crate::conf;
use crate::error::ApiError;
use salvo::http::header::AUTHORIZATION;
use salvo::prelude::*;
use std::sync::LazyLock;

/// JWT 认证中间件
#[derive(Clone)]
pub struct JwtAuthHandler;

#[handler]
impl JwtAuthHandler {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let path = req.uri().path();

        // 检查是否在忽略列表中
        match conf::get().auth().ignored(path) {
            Ok(true) => return,
            Ok(false) => {}
            Err(e) => {
                e.write_to_response(res);
                ctrl.skip_rest();
                return;
            }
        }

        let token = match req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        {
            Some(token) => token,
            None => {
                let err =
                    ApiError::Unauthenticated(String::from("Authorization请求头缺失或格式无效"));
                err.write_to_response(res);
                ctrl.skip_rest();
                return;
            }
        };

        match default_jwt().decode(token) {
            Ok(principal) => {
                req.extensions_mut().insert(principal);
            }
            Err(err) => {
                let api_err = ApiError::Internal(err);
                api_err.write_to_response(res);
                ctrl.skip_rest();
            }
        }
    }
}

static JWT_AUTH_HANDLER: LazyLock<JwtAuthHandler> = LazyLock::new(JwtAuthHandler::new);

impl JwtAuthHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get() -> &'static JwtAuthHandler {
        &JWT_AUTH_HANDLER
    }
}

/// 从 Request extensions 中提取 Principal
pub fn extract_principal(req: &Request) -> Result<Principal, ApiError> {
    req.extensions()
        .get::<Principal>()
        .cloned()
        .ok_or_else(|| ApiError::Unauthenticated(String::from("未找到认证信息")))
}
