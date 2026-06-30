use std::collections::HashMap;

/// Validates SIS config before system boot.
pub fn validate_config(config: &HashMap<String, String>) -> Result<(), String> {
    if !config.contains_key("init_mode") {
        return Err("Missing init_mode".to_string());
    }

    if config.get("init_mode") == Some(&"unsafe".to_string()) {
        return Err("Unsafe init mode not allowed".to_string());
    }

    Ok(())
}
