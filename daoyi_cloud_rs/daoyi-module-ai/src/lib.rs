//! Module placeholder for the Rust rewrite of daoyi-module-ai.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct AiModule;

impl Module for AiModule {
    fn name(&self) -> &'static str {
        "daoyi-module-ai"
    }
}
