pub trait Module {
    fn name(&self) -> &'static str;
}

pub trait ServiceModule: Module {
    fn init(&self) {}
}

pub fn describe_modules(modules: &[&dyn Module]) -> Vec<String> {
    modules
        .iter()
        .map(|module| module.name().to_string())
        .collect()
}
