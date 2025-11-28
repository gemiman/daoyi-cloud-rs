//! Module placeholder for the Rust rewrite of daoyi-module-report.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct ReportModule;

impl Module for ReportModule {
    fn name(&self) -> &'static str {
        "daoyi-module-report"
    }
}
