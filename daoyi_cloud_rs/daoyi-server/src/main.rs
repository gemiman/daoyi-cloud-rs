use daoyi_framework::{
    module::{Module, describe_modules},
    telemetry,
};
use daoyi_module_ai::AiModule;
use daoyi_module_bpm::BpmModule;
use daoyi_module_crm::CrmModule;
use daoyi_module_erp::ErpModule;
use daoyi_module_infra::InfraModule;
use daoyi_module_iot::IotModule;
use daoyi_module_mall::MallModule;
use daoyi_module_member::MemberModule;
use daoyi_module_mp::MpModule;
use daoyi_module_pay::PayModule;
use daoyi_module_report::ReportModule;
use daoyi_module_system::SystemModule;
use salvo::prelude::*;

#[handler]
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    telemetry::init_tracing("daoyi-server");

    let modules: [&dyn Module; 12] = [
        &SystemModule,
        &InfraModule,
        &MemberModule,
        &BpmModule,
        &PayModule,
        &ReportModule,
        &MpModule,
        &MallModule,
        &CrmModule,
        &ErpModule,
        &AiModule,
        &IotModule,
    ];
    tracing::info!(modules = ?describe_modules(&modules), "bootstrapping core server");

    let router = Router::new()
        .push(Router::with_path("/healthz").get(health))
        .push(
            Router::with_path("admin-api")
                .push(SystemModule::router())
                .push(InfraModule::router()),
        );
    let acceptor = TcpListener::new("0.0.0.0:18081").bind().await;
    Server::new(acceptor).serve(router).await;
}
