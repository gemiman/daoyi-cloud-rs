use chrono::{Duration, Utc};
use salvo::http::Method as HttpMethod;
use salvo::prelude::*;
use serde::Serialize;
use serde_json::{Value, json};

use daoyi_framework::web::{CaptchaResponse, CommonResult};

#[derive(Clone)]
struct RouteSpec {
    method: HttpMethod,
    path: &'static str,
}

impl RouteSpec {
    const fn new(method: HttpMethod, path: &'static str) -> Self {
        Self { method, path }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthLoginResp {
    user_id: u64,
    access_token: String,
    refresh_token: String,
    expires_time: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    id: u64,
    nickname: String,
    avatar: String,
    dept_id: u64,
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MenuInfo {
    id: u64,
    parent_id: u64,
    name: String,
    path: String,
    component: String,
    component_name: String,
    icon: String,
    visible: bool,
    keep_alive: bool,
    always_show: bool,
    children: Vec<MenuInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PermissionInfo {
    user: UserInfo,
    roles: Vec<String>,
    permissions: Vec<String>,
    menus: Vec<MenuInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CaptchaPayload {
    captcha_type: String,
    token: String,
    captcha_id: String,
    original_image_base64: String,
    jigsaw_image_base64: String,
    point: (u16, u16),
}

pub fn system_router() -> Router {
    let special = Router::new()
        .push(Router::with_path("/system/auth/login").post(auth_login))
        .push(Router::with_path("/system/auth/refresh-token").post(refresh_token))
        .push(Router::with_path("/system/auth/logout").post(simple_true))
        .push(Router::with_path("/system/auth/get-permission-info").get(permission_info))
        .push(Router::with_path("/system/captcha/get").post(captcha_get))
        .push(Router::with_path("/system/captcha/check").post(captcha_check));

    ROUTES
        .iter()
        .fold(special, |router, spec| add_route(router, spec))
}

fn add_route(router: Router, spec: &RouteSpec) -> Router {
    let path = normalize_path(spec.path);
    match spec.method {
        HttpMethod::GET => router.push(Router::with_path(path).get(generic_ok)),
        HttpMethod::POST => router.push(Router::with_path(path).post(generic_ok)),
        HttpMethod::PUT => router.push(Router::with_path(path).put(generic_ok)),
        HttpMethod::DELETE => router.push(Router::with_path(path).delete(generic_ok)),
        _ => router,
    }
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    }
}

#[handler]
async fn auth_login() -> Json<CommonResult<AuthLoginResp>> {
    Json(CommonResult::success(build_auth_resp()))
}

#[handler]
async fn refresh_token() -> Json<CommonResult<AuthLoginResp>> {
    Json(CommonResult::success(build_auth_resp()))
}

#[handler]
async fn simple_true() -> Json<CommonResult<bool>> {
    Json(CommonResult::success(true))
}

fn build_auth_resp() -> AuthLoginResp {
    let now = Utc::now();
    AuthLoginResp {
        user_id: 1,
        access_token: "mock-access-token".to_string(),
        refresh_token: "mock-refresh-token".to_string(),
        expires_time: now + Duration::hours(4),
    }
}

#[handler]
async fn permission_info() -> Json<CommonResult<PermissionInfo>> {
    let menu_dashboard = MenuInfo {
        id: 1,
        parent_id: 0,
        name: "Dashboard".into(),
        path: "/dashboard".into(),
        component: "dashboard/Analysis".into(),
        component_name: "DashboardAnalysis".into(),
        icon: "ion:grid-outline".into(),
        visible: true,
        keep_alive: true,
        always_show: true,
        children: vec![],
    };
    let menu_system = MenuInfo {
        id: 2,
        parent_id: 0,
        name: "System".into(),
        path: "/system".into(),
        component: "LAYOUT".into(),
        component_name: "SystemLayout".into(),
        icon: "ion:settings-outline".into(),
        visible: true,
        keep_alive: true,
        always_show: true,
        children: vec![MenuInfo {
            id: 3,
            parent_id: 2,
            name: "User Management".into(),
            path: "user".into(),
            component: "system/user/index".into(),
            component_name: "SystemUser".into(),
            icon: "ion:person-outline".into(),
            visible: true,
            keep_alive: true,
            always_show: false,
            children: vec![],
        }],
    };

    let info = PermissionInfo {
        user: UserInfo {
            id: 1,
            nickname: "Admin".into(),
            avatar: "https://dummyimage.com/120x120/1890ff/ffffff&text=DAOYI".into(),
            dept_id: 1,
            username: "admin".into(),
            email: "admin@example.com".into(),
        },
        roles: vec!["super_admin".into()],
        permissions: vec!["*:*:*".into()],
        menus: vec![menu_dashboard, menu_system],
    };
    Json(CommonResult::success(info))
}

#[handler]
async fn captcha_get() -> Json<CaptchaResponse<CaptchaPayload>> {
    let payload = CaptchaPayload {
        captcha_type: "blockPuzzle".into(),
        token: "mock-captcha-token".into(),
        captcha_id: "mock-captcha-id".into(),
        original_image_base64: "".into(),
        jigsaw_image_base64: "".into(),
        point: (15, 8),
    };
    Json(CaptchaResponse::success(payload))
}

#[handler]
async fn captcha_check() -> Json<CaptchaResponse<Value>> {
    Json(CaptchaResponse::success(json!({"result": true})))
}

#[handler]
async fn generic_ok(req: &mut Request) -> Json<CommonResult<Value>> {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    Json(CommonResult::success(json!({
        "mock": true,
        "path": path,
        "method": method,
    })))
}

// Auto-generated route specs from Java controllers to keep 1:1 endpoints.
const ROUTES: &[RouteSpec] = &[
    RouteSpec::new(HttpMethod::DELETE, "/system/dept/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/deptdelete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/dict-data/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/dict-data/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/dict-type/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/dict-type/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/mail-account/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/mail-account/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/mail-template/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/mail-template/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/menu/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/menu/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/notice/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/notice/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/notify-template/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/notify-template/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/oauth2-client/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/oauth2-client/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/oauth2-token/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/oauth2-token/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/oauth2/token"),
    RouteSpec::new(HttpMethod::DELETE, "/system/post/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/role/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/role/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/sms-template/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/sms-template/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/social-client/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/social-client/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/social-user/unbind"),
    RouteSpec::new(HttpMethod::DELETE, "/system/tenant-package/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/tenant-package/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/tenant/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/tenant/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/user/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/user/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/system/sms-channel/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/system/sms-channel/delete-list"),
    RouteSpec::new(HttpMethod::GET, "/system/area/get-by-ip"),
    RouteSpec::new(HttpMethod::GET, "/system/area/tree"),
    RouteSpec::new(HttpMethod::GET, "/system/auth/social-auth-redirect"),
    RouteSpec::new(HttpMethod::GET, "/system/dept/get"),
    RouteSpec::new(HttpMethod::GET, "/system/dept/list"),
    RouteSpec::new(HttpMethod::GET, "/system/dept/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-data/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-data/get"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-data/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-data/page"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-data/type"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-type/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-type/get"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-type/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/dict-type/page"),
    RouteSpec::new(HttpMethod::GET, "/system/login-log/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/login-log/page"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-account/get"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-account/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-account/page"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-log/get"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-log/page"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-template/get"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-template/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/mail-template/page"),
    RouteSpec::new(HttpMethod::GET, "/system/menu/get"),
    RouteSpec::new(HttpMethod::GET, "/system/menu/list"),
    RouteSpec::new(HttpMethod::GET, "/system/menu/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/notice/get"),
    RouteSpec::new(HttpMethod::GET, "/system/notice/page"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-message/get"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-message/get-unread-count"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-message/get-unread-list"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-message/my-page"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-message/page"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-template/get"),
    RouteSpec::new(HttpMethod::GET, "/system/notify-template/page"),
    RouteSpec::new(HttpMethod::GET, "/system/oauth2-client/get"),
    RouteSpec::new(HttpMethod::GET, "/system/oauth2-client/page"),
    RouteSpec::new(HttpMethod::GET, "/system/oauth2-token/page"),
    RouteSpec::new(HttpMethod::GET, "/system/oauth2/authorize"),
    RouteSpec::new(HttpMethod::GET, "/system/oauth2/user/get"),
    RouteSpec::new(HttpMethod::GET, "/system/operate-log/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/operate-log/page"),
    RouteSpec::new(HttpMethod::GET, "/system/permission/list-role-menus"),
    RouteSpec::new(HttpMethod::GET, "/system/permission/list-user-roles"),
    RouteSpec::new(HttpMethod::GET, "/system/post/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/post/get"),
    RouteSpec::new(HttpMethod::GET, "/system/post/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/post/page"),
    RouteSpec::new(HttpMethod::GET, "/system/role/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/role/get"),
    RouteSpec::new(HttpMethod::GET, "/system/role/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/role/page"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-log/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-log/page"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-template/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-template/get"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-template/page"),
    RouteSpec::new(HttpMethod::GET, "/system/social-client/get"),
    RouteSpec::new(HttpMethod::GET, "/system/social-client/page"),
    RouteSpec::new(HttpMethod::GET, "/system/social-user/get"),
    RouteSpec::new(HttpMethod::GET, "/system/social-user/get-bind-list"),
    RouteSpec::new(HttpMethod::GET, "/system/social-user/page"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant-package/get"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant-package/get-simple-list"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant-package/page"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant/get"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant/get-by-website"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant/get-id-by-name"),
    RouteSpec::new(HttpMethod::GET, "/system/tenant/page"),
    RouteSpec::new(HttpMethod::GET, "/system/tenantsimple-list"),
    RouteSpec::new(HttpMethod::GET, "/system/user/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/system/user/get"),
    RouteSpec::new(HttpMethod::GET, "/system/user/get-import-template"),
    RouteSpec::new(HttpMethod::GET, "/system/user/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/user/page"),
    RouteSpec::new(HttpMethod::GET, "/system/user/profile/get"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-channel/get"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-channel/list-all-simple"),
    RouteSpec::new(HttpMethod::GET, "/system/sms-channel/page"),
    RouteSpec::new(HttpMethod::POST, "/system/auth/register"),
    RouteSpec::new(HttpMethod::POST, "/system/auth/reset-password"),
    RouteSpec::new(HttpMethod::POST, "/system/auth/send-sms-code"),
    RouteSpec::new(HttpMethod::POST, "/system/auth/sms-login"),
    RouteSpec::new(HttpMethod::POST, "/system/auth/social-login"),
    RouteSpec::new(HttpMethod::POST, "/system/deptcreate"),
    RouteSpec::new(HttpMethod::POST, "/system/dict-data/create"),
    RouteSpec::new(HttpMethod::POST, "/system/dict-type/create"),
    RouteSpec::new(HttpMethod::POST, "/system/mail-account/create"),
    RouteSpec::new(HttpMethod::POST, "/system/mail-template/create"),
    RouteSpec::new(HttpMethod::POST, "/system/mail-template/send-mail"),
    RouteSpec::new(HttpMethod::POST, "/system/menu/create"),
    RouteSpec::new(HttpMethod::POST, "/system/notice/create"),
    RouteSpec::new(HttpMethod::POST, "/system/notice/push"),
    RouteSpec::new(HttpMethod::POST, "/system/notify-template/create"),
    RouteSpec::new(HttpMethod::POST, "/system/notify-template/send-notify"),
    RouteSpec::new(HttpMethod::POST, "/system/oauth2-client/create"),
    RouteSpec::new(HttpMethod::POST, "/system/oauth2/authorize"),
    RouteSpec::new(HttpMethod::POST, "/system/oauth2/check-token"),
    RouteSpec::new(HttpMethod::POST, "/system/oauth2/token"),
    RouteSpec::new(
        HttpMethod::POST,
        "/system/permission/assign-role-data-scope",
    ),
    RouteSpec::new(HttpMethod::POST, "/system/permission/assign-role-menu"),
    RouteSpec::new(HttpMethod::POST, "/system/permission/assign-user-role"),
    RouteSpec::new(HttpMethod::POST, "/system/post/create"),
    RouteSpec::new(HttpMethod::POST, "/system/role/create"),
    RouteSpec::new(HttpMethod::POST, "/system/sms-template/create"),
    RouteSpec::new(HttpMethod::POST, "/system/sms-template/send-sms"),
    RouteSpec::new(HttpMethod::POST, "/system/sms/callback/aliyun"),
    RouteSpec::new(HttpMethod::POST, "/system/sms/callback/huawei"),
    RouteSpec::new(HttpMethod::POST, "/system/sms/callback/qiniu"),
    RouteSpec::new(HttpMethod::POST, "/system/sms/callback/tencent"),
    RouteSpec::new(HttpMethod::POST, "/system/social-client/create"),
    RouteSpec::new(
        HttpMethod::POST,
        "/system/social-client/send-subscribe-message",
    ),
    RouteSpec::new(HttpMethod::POST, "/system/social-user/bind"),
    RouteSpec::new(HttpMethod::POST, "/system/tenant-package/create"),
    RouteSpec::new(HttpMethod::POST, "/system/tenant/create"),
    RouteSpec::new(HttpMethod::POST, "/system/user/create"),
    RouteSpec::new(HttpMethod::POST, "/system/user/import"),
    RouteSpec::new(HttpMethod::POST, "/system/sms-channel/create"),
    RouteSpec::new(HttpMethod::PUT, "/system/deptupdate"),
    RouteSpec::new(HttpMethod::PUT, "/system/dict-data/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/dict-type/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/mail-account/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/mail-template/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/menu/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/notice/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/notify-message/update-all-read"),
    RouteSpec::new(HttpMethod::PUT, "/system/notify-message/update-read"),
    RouteSpec::new(HttpMethod::PUT, "/system/notify-template/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/oauth2-client/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/oauth2/user/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/post/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/role/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/sms-template/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/social-client/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/tenant-package/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/tenant/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/user/profile/update"),
    RouteSpec::new(HttpMethod::PUT, "/system/user/profile/update-password"),
    RouteSpec::new(HttpMethod::PUT, "/system/user/update-password"),
    RouteSpec::new(HttpMethod::PUT, "/system/user/update-status"),
    RouteSpec::new(HttpMethod::PUT, "/system/userupdate"),
    RouteSpec::new(HttpMethod::PUT, "/system/sms-channel/update"),
];
