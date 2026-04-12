use crate::auth::jwt::{Principal, default_jwt};
use crate::conf;
use crate::error::ApiError;
use crate::response::ApiResponse;
use salvo::http::StatusCode;
use salvo::http::header::AUTHORIZATION;
use salvo::prelude::*;
use salvo::writing::Json;
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
        match conf::get().auth().ignored(path) {
            Ok(true) => {
                return;
            }
            Ok(false) => {}
            Err(e) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(ApiResponse::<()>::err_msg(e.to_string())));
                ctrl.skip_rest();
                return;
            }
        }

        let token = req
            .headers()
            .get(AUTHORIZATION)
            .map(|value| -> Result<_, ApiError> {
                let token = value
                    .to_str()
                    .map_err(|_| {
                        ApiError::Unauthenticated(String::from("Authorization请求头无效"))
                    })?
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| {
                        ApiError::Unauthenticated(String::from("Authorization请求头格式无效"))
                    })?;
                Ok(token)
            })
            .transpose();

        match token {
            Ok(Some(t)) => match default_jwt().decode(t) {
                Ok(principal) => {
                    req.extensions_mut().insert(principal);
                }
                Err(err) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(Json(ApiResponse::<()>::err_msg(err.to_string())));
                    ctrl.skip_rest();
                    return;
                }
            },
            Ok(None) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiResponse::<()>::err_msg("Authorization请求头缺失")));
                ctrl.skip_rest();
                return;
            }
            Err(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiResponse::<()>::err_msg(e.to_string())));
                ctrl.skip_rest();
                return;
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
