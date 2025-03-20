//! Config deserializer for [`Value`]
//!
//! [`Value`]: serde_json::Value

use std::collections::HashMap;

use serde_json::Value;

/// Flattens a [`Value`] into a flat hashmap
///
/// [`Value`]: serde_json::Value
fn flatten(hm: &mut HashMap<String, Value>, key: String, value: Value) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                flatten(hm, format!("{key}.{k}"), v);
            }
        }
        _ => {
            hm.insert(key, value);
        }
    }
}

/// Deserializes a [`Value`] into a config Hashmap
///
/// [`Value`]: serde_json::Value
pub fn deserialize(value: Value) -> HashMap<String, Value> {
    let mut hm = HashMap::new();
    flatten(&mut hm, String::new(), value);
    hm
}
