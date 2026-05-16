use crate::auth::jwt::middleware::extract_principal;
use crate::error::ApiError;
use salvo::prelude::*;

/// RBAC 权限检查中间件
///
/// ```ignore
/// Router::with_path("users")
///     .hoop(RequirePermission::new("user:delete"))
///     .delete(delete_user)
/// ```
#[derive(Clone)]
pub struct RequirePermission {
    permission: &'static str,
}

impl RequirePermission {
    pub const fn new(permission: &'static str) -> Self {
        Self { permission }
    }
}

#[handler]
impl RequirePermission {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        match extract_principal(req) {
            Ok(principal) => {
                if !principal.has_permission(self.permission) {
                    ApiError::Forbidden(format!("缺少权限: {}", self.permission))
                        .write_to_response(res);
                    ctrl.skip_rest();
                }
                // 权限通过，继续
            }
            Err(e) => {
                e.write_to_response(res);
                ctrl.skip_rest();
            }
        }
    }
}

/// 角色检查中间件
#[derive(Clone)]
pub struct RequireRole {
    role: &'static str,
}

impl RequireRole {
    pub const fn new(role: &'static str) -> Self {
        Self { role }
    }
}

#[handler]
impl RequireRole {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        match extract_principal(req) {
            Ok(principal) => {
                if !principal.has_role(self.role) {
                    ApiError::Forbidden(format!("需要角色: {}", self.role)).write_to_response(res);
                    ctrl.skip_rest();
                }
            }
            Err(e) => {
                e.write_to_response(res);
                ctrl.skip_rest();
            }
        }
    }
}
