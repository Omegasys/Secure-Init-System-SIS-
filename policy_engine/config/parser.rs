use std::collections::HashMap;

/// Simple key=value config parser (MVP SIS config format)
pub fn parse_config(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            map.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        }
    }

    map
}
