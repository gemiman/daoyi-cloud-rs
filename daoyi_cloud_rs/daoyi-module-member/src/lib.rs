//! Module placeholder for the Rust rewrite of daoyi-module-member.
#![allow(dead_code)]

use daoyi_framework::module::Module;

pub struct MemberModule;

impl Module for MemberModule {
    fn name(&self) -> &'static str {
        "daoyi-module-member"
    }
}
