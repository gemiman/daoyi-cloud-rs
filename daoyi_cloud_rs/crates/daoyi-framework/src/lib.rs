pub mod prelude {
    pub use anyhow::Result;
}

pub fn init_framework() {
    tracing::info!(target = "daoyi-framework", "framework initialized (skeleton)");
}
