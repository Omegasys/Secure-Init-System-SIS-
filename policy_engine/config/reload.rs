use std::collections::HashMap;

/// Hot reload config (simplified SIS behavior)
pub fn reload_config(old: &HashMap<String, String>, new: &HashMap<String, String>) {
    println!("[SIS CONFIG] Reloading configuration...");

    for (k, v) in new {
        let old_val = old.get(k);

        if old_val != Some(v) {
            println!(" - updated {} = {}", k, v);
        }
    }

    println!("[SIS CONFIG] Reload complete");
}
