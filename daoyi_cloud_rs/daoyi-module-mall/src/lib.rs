//! Module placeholder for the Rust rewrite of daoyi-module-mall.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct MallModule;

impl Module for MallModule {
    fn name(&self) -> &'static str {
        "daoyi-module-mall"
    }
}
