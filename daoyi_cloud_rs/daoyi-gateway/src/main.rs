use daoyi_framework::{
    module::{Module, describe_modules},
    telemetry,
};
use daoyi_module_infra::InfraModule;
use daoyi_module_system::SystemModule;
use salvo::prelude::*;

#[handler]
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    telemetry::init_tracing("daoyi-gateway");

    let modules: [&dyn Module; 2] = [&SystemModule, &InfraModule];
    tracing::info!(modules = ?describe_modules(&modules), "bootstrapping gateway");

    let router = Router::new()
        .push(Router::with_path("/healthz").get(health))
        .push(
            Router::with_path("admin-api")
                .push(SystemModule::router())
                .push(InfraModule::router()),
        );

    let acceptor = TcpListener::new("0.0.0.0:18080").bind().await;
    Server::new(acceptor).serve(router).await;
}
