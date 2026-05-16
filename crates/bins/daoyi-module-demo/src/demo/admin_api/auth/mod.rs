use daoyi_cloud_common::auth::jwt::Principal;
use daoyi_cloud_common::auth::jwt::middleware::extract_principal;
use daoyi_cloud_common::context::AppContext;
use daoyi_cloud_common::extract;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_entity_demo::demo::models::auth::LoginParams;
use daoyi_entity_demo::demo::models::auth::LoginResult;
use daoyi_entity_demo::demo::service::auth_service;
use salvo::oapi::endpoint;
use salvo::prelude::*;

pub fn create_router() -> Router {
    Router::new()
        .push(Router::with_path("/login").post(login))
        .push(Router::with_path("/user-info").get(get_user_info))
}

/// 用户登录
#[endpoint(
    tags("认证管理"),
    operation_id = "login",
    summary = "用户登录",
    description = "通过账号密码登录系统，返回 JWT 访问令牌",
    request_body = LoginParams,
    responses(
        (status_code = 200, description = "登录成功，返回访问令牌", body = ApiResponse<LoginResult>)
    )
)]
async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let params = match extract::extract_valid_json::<LoginParams>(req, depot).await {
        Ok(p) => p,
        Err(e) => {
            e.write_to_response(res);
            return;
        }
    };
    // 从 Depot 获取 AppContext，再获取 db 连接传给 Service（DI 模式）
    let db = match AppContext::from_depot(depot) {
        Some(ctx) => &ctx.db,
        None => {
            daoyi_cloud_common::error::ApiError::Internal(anyhow::anyhow!("AppContext 未注入"))
                .write_to_response(res);
            return;
        }
    };
    match auth_service::login(db, params).await {
        Ok(result) => daoyi_cloud_common::json_ok!(res, result),
        Err(e) => e.write_to_response(res),
    }
}

/// 获取当前用户信息
#[endpoint(
    tags("认证管理"),
    operation_id = "getUserInfo",
    summary = "获取当前用户信息",
    description = "根据 JWT 令牌获取当前登录用户的基本信息",
    responses(
        (status_code = 200, description = "获取成功，返回用户信息", body = ApiResponse<Principal>)
    )
)]
async fn get_user_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // DI 示例：AppContext 已注入到 Depot
    let _ = AppContext::from_depot(depot);
    match extract_principal(req) {
        Ok(principal) => daoyi_cloud_common::json_ok!(res, principal),
        Err(e) => e.write_to_response(res),
    }
}
