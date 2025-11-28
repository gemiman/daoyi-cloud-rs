//! System module mock implementation for running without DB/Redis.

pub mod routes;

use daoyi_framework::module::Module;

pub struct SystemModule;

impl Module for SystemModule {
    fn name(&self) -> &'static str {
        "daoyi-module-system"
    }
}

impl SystemModule {
    pub fn router() -> salvo::Router {
        routes::system_router()
    }
}
