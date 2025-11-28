//! Module placeholder for the Rust rewrite of daoyi-module-bpm.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct BpmModule;

impl Module for BpmModule {
    fn name(&self) -> &'static str {
        "daoyi-module-bpm"
    }
}
