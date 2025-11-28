//! Module placeholder for the Rust rewrite of daoyi-module-pay.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct PayModule;

impl Module for PayModule {
    fn name(&self) -> &'static str {
        "daoyi-module-pay"
    }
}
