//! Module placeholder for the Rust rewrite of daoyi-module-iot.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct IotModule;

impl Module for IotModule {
    fn name(&self) -> &'static str {
        "daoyi-module-iot"
    }
}
