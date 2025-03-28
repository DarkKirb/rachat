//! Config serializer for [`Value`]
//!
//! [`Value`]: serde_json::Value

use std::collections::HashMap;

use eyre::Result;
use serde_json::{Map, Value};

/// Inserts a value at a specific location
///
/// # Errors
/// This funciton returns an error if a map would overwrite a more descriptive value
fn insert<'a>(
    mut path_segments: impl Iterator<Item = &'a str>,
    root: &mut Value,
    value: Value,
) -> Result<()> {
    if let Some(segment) = path_segments.next() {
        match root {
            Value::Object(map) => {
                if !map.contains_key(segment) {
                    map.insert(segment.to_string(), Value::Object(Map::default()));
                };

                insert(path_segments, &mut map[segment], value)?;
            }
            _ => {
                eyre::bail!("{root:?} is not a map");
            }
        }
    } else {
        match root {
            Value::Object(map) if map.is_empty() => {
                *root = value;
            }
            _ => {
                eyre::bail!("Cannot overwrite non-empty object: {root:?}");
            }
        }
    }
    Ok(())
}

/// Serializes a settings-value map into a [`Value`]
///
/// # Errors
/// This function returns an error if the value cannot be serialized.
///
/// [`Value`]: serde_json::Value
pub fn serialize(value: &HashMap<String, Value>) -> Result<Value> {
    let mut root = Value::Object(Map::default());
    for (key, value) in value {
        insert(key.split('.'), &mut root, value.clone())?;
    }
    Ok(root)
}
