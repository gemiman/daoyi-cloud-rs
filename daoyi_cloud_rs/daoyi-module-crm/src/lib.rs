//! Module placeholder for the Rust rewrite of daoyi-module-crm.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct CrmModule;

impl Module for CrmModule {
    fn name(&self) -> &'static str {
        "daoyi-module-crm"
    }
}
