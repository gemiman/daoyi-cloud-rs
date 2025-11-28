//! Module placeholder for the Rust rewrite of daoyi-module-erp.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct ErpModule;

impl Module for ErpModule {
    fn name(&self) -> &'static str {
        "daoyi-module-erp"
    }
}
