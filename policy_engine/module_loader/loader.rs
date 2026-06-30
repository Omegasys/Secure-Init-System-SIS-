use std::collections::HashMap;
use crate::registry::ModuleRegistry;
use crate::verification::verify_module;

pub struct ModuleLoader {
    registry: ModuleRegistry,
}

impl ModuleLoader {
    pub fn new(registry: ModuleRegistry) -> Self {
        Self { registry }
    }

    pub fn load_module(&mut self, name: &str) -> Result<(), String> {
        let module = self.registry.get(name)
            .ok_or("Module not found")?;

        // Step 1: verify module integrity
        if !verify_module(module) {
            return Err("Module verification failed".to_string());
        }

        // Step 2: simulate loading
        println!("[SIS MODULE] Loaded: {}", name);

        Ok(())
    }

    pub fn load_all(&mut self) -> Result<(), String> {
        for name in self.registry.all_modules() {
            self.load_module(&name)?;
        }
        Ok(())
    }
}
