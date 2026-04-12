use daoyi_cloud_common::extract;
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_entity_demo::demo::entity::sys_user::Model as SysUser;
use daoyi_entity_demo::demo::models::sys_user::{UserParams, UserQueryParams};
use daoyi_entity_demo::demo::service::sys_user_service;
use salvo::oapi::endpoint;
use salvo::prelude::*;

pub fn create_router() -> Router {
    Router::new()
        .push(Router::with_path("/").get(query_users))
        .push(Router::with_path("/page").get(find_page))
        .push(Router::with_path("/").post(create))
        .push(Router::with_path("/<id>").put(update))
        .push(Router::with_path("/<id>").get(get_user_by_id))
        .push(Router::with_path("/<id>").delete(delete))
}

/// 删除用户
#[endpoint(
    tags("用户管理"),
    operation_id = "deleteUser",
    summary = "删除用户",
    description = "根据用户ID删除指定用户",
    responses(
        (status_code = 200, description = "删除成功", body = ApiResponse<()>)
    )
)]
async fn delete(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i64 = match extract::extract_path_param(req, "id") {
        Ok(id) => id,
        Err(e) => {
            daoyi_cloud_common::response::write_error_response(res, e);
            return;
        }
    };
    match sys_user_service::delete_user_by_id(id).await {
        Ok(result) => daoyi_cloud_common::success!(res, result),
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}

/// 根据ID查询用户
#[endpoint(
    tags("用户管理"),
    operation_id = "getUserById",
    summary = "根据ID查询用户",
    description = "根据用户ID获取用户详细信息",
    responses(
        (status_code = 200, description = "查询成功，返回用户信息", body = ApiResponse<SysUser>)
    )
)]
async fn get_user_by_id(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i64 = match extract::extract_path_param(req, "id") {
        Ok(id) => id,
        Err(e) => {
            daoyi_cloud_common::response::write_error_response(res, e);
            return;
        }
    };
    match sys_user_service::get_user_by_id(id).await {
        Ok(user) => daoyi_cloud_common::success!(res, user),
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}

/// 更新用户
#[endpoint(
    tags("用户管理"),
    operation_id = "updateUser",
    summary = "更新用户",
    description = "根据用户ID更新用户信息",
    request_body = UserParams,
    responses(
        (status_code = 200, description = "更新成功", body = ApiResponse<SysUser>)
    )
)]
async fn update(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id: i64 = match extract::extract_path_param(req, "id") {
        Ok(id) => id,
        Err(e) => {
            daoyi_cloud_common::response::write_error_response(res, e);
            return;
        }
    };
    match extract::extract_valid_json::<UserParams>(req, depot).await {
        Ok(params) => match sys_user_service::update_user_by_id(id, params).await {
            Ok(result) => daoyi_cloud_common::success!(res, result),
            Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
        },
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}

/// 创建用户
#[endpoint(
    tags("用户管理"),
    operation_id = "createUser",
    summary = "创建用户",
    description = "创建新用户",
    request_body = UserParams,
    responses(
        (status_code = 200, description = "创建成功，返回用户信息", body = ApiResponse<SysUser>)
    )
)]
async fn create(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match extract::extract_valid_json::<UserParams>(req, depot).await {
        Ok(params) => match sys_user_service::create_user(params).await {
            Ok(user) => daoyi_cloud_common::success!(res, user),
            Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
        },
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}

/// 分页查询用户
#[endpoint(
    tags("用户管理"),
    operation_id = "findUserPage",
    summary = "分页查询用户",
    description = "根据关键词和分页参数分页查询用户列表",
    responses(
        (status_code = 200, description = "查询成功，返回分页结果", body = ApiResponse<PageResult<SysUser>>)
    )
)]
async fn find_page(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match extract::extract_valid_query::<UserQueryParams>(req, depot).await {
        Ok(params) => match sys_user_service::query_page(params).await {
            Ok(users) => daoyi_cloud_common::success!(res, users),
            Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
        },
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}

/// 查询所有用户
#[endpoint(
    tags("用户管理"),
    operation_id = "queryUsers",
    summary = "查询所有用户",
    description = "查询系统中所有用户列表",
    responses(
        (status_code = 200, description = "查询成功，返回用户列表", body = ApiResponse<Vec<SysUser>>)
    )
)]
async fn query_users(res: &mut Response) {
    match sys_user_service::query_users().await {
        Ok(users) => daoyi_cloud_common::success!(res, users),
        Err(e) => daoyi_cloud_common::response::write_error_response(res, e),
    }
}
