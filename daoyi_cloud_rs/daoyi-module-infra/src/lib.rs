//! Infra module mock implementation.

pub mod routes;

use daoyi_framework::module::Module;

pub struct InfraModule;

impl Module for InfraModule {
    fn name(&self) -> &'static str {
        "daoyi-module-infra"
    }
}

impl InfraModule {
    pub fn router() -> salvo::Router {
        routes::infra_router()
    }
}
