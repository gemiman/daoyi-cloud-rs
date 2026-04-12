use daoyi_cloud_common::app;
use daoyi_cloud_common::openapi::ApiDoc;
use daoyi_module_demo::demo;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (router, demo_api) = demo::create_router();
    let mut api = ApiDoc::openapi();
    api.merge(demo_api);
    app::run("demo", router, api).await
}
