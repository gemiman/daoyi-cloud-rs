//! Module placeholder for the Rust rewrite of daoyi-module-mp.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct MpModule;

impl Module for MpModule {
    fn name(&self) -> &'static str {
        "daoyi-module-mp"
    }
}
