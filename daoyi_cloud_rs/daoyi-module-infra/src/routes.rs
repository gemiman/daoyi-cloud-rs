use salvo::http::Method as HttpMethod;
use salvo::prelude::*;
use serde::Serialize;
use serde_json::{Value, json};

use daoyi_framework::web::CommonResult;

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

pub fn infra_router() -> Router {
    let specials = Router::new()
        .push(Router::with_path("/infra/config/get-value-by-key").get(get_value_by_key))
        .push(Router::with_path("/infra/file/upload").post(upload_mock))
        .push(Router::with_path("/infra/file/presigned-url").get(presigned_url))
        .push(Router::with_path("/infra/file/create").post(upload_mock))
        .push(Router::with_path("/infra/redis/get-monitor-info").get(redis_monitor));

    ROUTES
        .iter()
        .fold(specials, |router, spec| add_route(router, spec))
}

fn normalize_path(path: &str) -> String {
    let mut p = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };
    if p.contains('{') {
        p = p
            .replace("{configId}", "<config_id>")
            .replace("**", "<**rest>");
    }
    p
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

#[handler]
async fn generic_ok(req: &mut Request) -> Json<CommonResult<Value>> {
    Json(CommonResult::success(json!({
        "mock": true,
        "path": req.uri().path(),
        "method": req.method().to_string(),
    })))
}

#[handler]
async fn get_value_by_key(req: &mut Request) -> Json<CommonResult<String>> {
    let key = req.query::<String>("key").unwrap_or_default();
    let value = match key.as_str() {
        "file.domain" => "http://localhost:18080".to_string(),
        _ => String::new(),
    };
    Json(CommonResult::success(value))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileResp {
    id: String,
    url: String,
}

#[handler]
async fn upload_mock() -> Json<CommonResult<FileResp>> {
    Json(CommonResult::success(FileResp {
        id: "mock-file-id".into(),
        url: "http://localhost:18080/mock-file".into(),
    }))
}

#[handler]
async fn presigned_url() -> Json<CommonResult<String>> {
    Json(CommonResult::success(
        "http://localhost:18080/mock-file".into(),
    ))
}

#[handler]
async fn redis_monitor() -> Json<CommonResult<Value>> {
    Json(CommonResult::success(json!({
        "info": "mock",
        "connected": false
    })))
}

// Auto-generated route specs from Java controllers to keep 1:1 endpoints.
const ROUTES: &[RouteSpec] = &[
    RouteSpec::new(HttpMethod::DELETE, "/infra/codegen/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/codegen/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/config/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/config/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/data-source-config/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/data-source-config/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo01-contact/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo01-contact/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo02-category/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo03-student-erp/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo03-student-erp/delete-list"),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-erp/demo03-course/delete",
    ),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-erp/demo03-course/delete-list",
    ),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-erp/demo03-grade/delete",
    ),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-erp/demo03-grade/delete-list",
    ),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo03-student-inner/delete"),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-inner/delete-list",
    ),
    RouteSpec::new(HttpMethod::DELETE, "/infra/demo03-student-normal/delete"),
    RouteSpec::new(
        HttpMethod::DELETE,
        "/infra/demo03-student-normal/delete-list",
    ),
    RouteSpec::new(HttpMethod::DELETE, "/infra/file-config/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/file-config/delete-list"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/file/delete"),
    RouteSpec::new(HttpMethod::DELETE, "/infra/file/delete-list"),
    RouteSpec::new(HttpMethod::GET, "/infra/api-access-log/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/api-access-log/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/api-error-log/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/api-error-log/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/db/table/list"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/detail"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/download"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/preview"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/table/list"),
    RouteSpec::new(HttpMethod::GET, "/infra/codegen/table/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/config/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/config/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/config/get-value-by-key"),
    RouteSpec::new(HttpMethod::GET, "/infra/config/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/data-source-config/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/data-source-config/list"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo01-contact/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo01-contact/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo01-contact/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo02-category/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo02-category/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo02-category/list"),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-erp/demo03-course/get",
    ),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-erp/demo03-course/page",
    ),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-erp/demo03-grade/get",
    ),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-erp/demo03-grade/page",
    ),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-erp/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-erp/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-erp/page"),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-inner/demo03-course/list-by-student-id",
    ),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-inner/demo03-grade/get-by-student-id",
    ),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-inner/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-inner/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-inner/page"),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-normal/demo03-course/list-by-student-id",
    ),
    RouteSpec::new(
        HttpMethod::GET,
        "/infra/demo03-student-normal/demo03-grade/get-by-student-id",
    ),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-normal/export-excel"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-normal/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/demo03-student-normal/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/file-config/get"),
    RouteSpec::new(HttpMethod::GET, "/infra/file-config/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/file-config/test"),
    RouteSpec::new(HttpMethod::GET, "/infra/file/page"),
    RouteSpec::new(HttpMethod::GET, "/infra/file/presigned-url"),
    RouteSpec::new(HttpMethod::GET, "/infra/file/{configId}/get/**"),
    RouteSpec::new(HttpMethod::GET, "/infra/redis/get-monitor-info"),
    RouteSpec::new(HttpMethod::POST, "/infra/codegen/create-list"),
    RouteSpec::new(HttpMethod::POST, "/infra/config/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/data-source-config/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/demo01-contact/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/demo02-category/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/demo03-student-erp/create"),
    RouteSpec::new(
        HttpMethod::POST,
        "/infra/demo03-student-erp/demo03-course/create",
    ),
    RouteSpec::new(
        HttpMethod::POST,
        "/infra/demo03-student-erp/demo03-grade/create",
    ),
    RouteSpec::new(HttpMethod::POST, "/infra/demo03-student-inner/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/demo03-student-normal/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/file-config/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/file/create"),
    RouteSpec::new(HttpMethod::POST, "/infra/file/upload"),
    RouteSpec::new(HttpMethod::PUT, "/infra/api-error-log/update-status"),
    RouteSpec::new(HttpMethod::PUT, "/infra/codegen/sync-from-db"),
    RouteSpec::new(HttpMethod::PUT, "/infra/codegen/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/config/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/data-source-config/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/demo01-contact/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/demo02-category/update"),
    RouteSpec::new(
        HttpMethod::PUT,
        "/infra/demo03-student-erp/demo03-course/update",
    ),
    RouteSpec::new(
        HttpMethod::PUT,
        "/infra/demo03-student-erp/demo03-grade/update",
    ),
    RouteSpec::new(HttpMethod::PUT, "/infra/demo03-student-erp/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/demo03-student-inner/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/demo03-student-normal/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/file-config/update"),
    RouteSpec::new(HttpMethod::PUT, "/infra/file-config/update-master"),
];
