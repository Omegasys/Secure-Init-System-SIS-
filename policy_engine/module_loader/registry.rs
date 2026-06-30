use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub signed: bool,
    pub hash: String,
}

pub struct ModuleRegistry {
    modules: HashMap<String, Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Module) {
        self.modules.insert(module.name.clone(), module);
    }

    pub fn get(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    pub fn all_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
}
