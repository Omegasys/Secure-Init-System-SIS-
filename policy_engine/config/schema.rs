/// SIS configuration schema definition (MVP version)

#[derive(Debug)]
pub struct SisConfig {
    pub init_mode: String,
    pub log_level: String,
    pub security_profile: String,
    pub module_path: String,
}

impl SisConfig {
    pub fn default() -> Self {
        Self {
            init_mode: "secure".to_string(),
            log_level: "info".to_string(),
            security_profile: "high".to_string(),
            module_path: "/usr/lib/sis/modules".to_string(),
        }
    }

    pub fn from_map(map: &std::collections::HashMap<String, String>) -> Self {
        Self {
            init_mode: map.get("init_mode").cloned().unwrap_or("secure".into()),
            log_level: map.get("log_level").cloned().unwrap_or("info".into()),
            security_profile: map.get("security_profile").cloned().unwrap_or("high".into()),
            module_path: map.get("module_path").cloned().unwrap_or("/usr/lib/sis/modules".into()),
        }
    }
}
