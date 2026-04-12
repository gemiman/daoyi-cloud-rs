use daoyi_module_demo::demo;
use salvo::prelude::*;

pub fn create_router() -> Router {
    let demo_router = demo::create_router();
    Router::new().push(demo_router)
}
