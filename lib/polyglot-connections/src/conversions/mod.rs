use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::error::Error;

/// Converts any serializable struct to a JSON string.
pub fn to_json<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    let json_str = serde_json::to_string(value)?;
    Ok(json_str)
}

/// Converts a JSON string back into a deserializable struct.
pub fn from_json<'a, T: Deserialize<'a>>(json_str: &'a str) -> Result<T, Box<dyn Error>> {
    let value = serde_json::from_str(json_str)?;
    Ok(value)
}

/// Converts any serializable struct to a YAML string.
pub fn to_yaml<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    let yaml_str = serde_yaml::to_string(value)?;
    Ok(yaml_str)
}

/// Converts a YAML string back into a deserializable struct.
pub fn from_yaml<'a, T: Deserialize<'a>>(yaml_str: &'a str) -> Result<T, Box<dyn Error>> {
    let value = serde_yaml::from_str(yaml_str)?;
    Ok(value)
}

/// Converts any serializable struct to a TOON string format.
pub fn to_toon<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    let toon_str = toon_format::encode_default(value).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(toon_str)
}

/// Converts a TOON string back into a deserializable struct.
pub fn from_toon<T: serde::de::DeserializeOwned>(toon_str: &str) -> Result<T, Box<dyn Error>> {
    let value = toon_format::decode_default(toon_str).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(value)
}

/// Converts any serializable struct to a serde_json::Value, which is inherently compatible
/// with PostgreSQL JSONB types via sqlx.
pub fn to_jsonb<T: Serialize>(value: &T) -> Result<JsonValue, Box<dyn Error>> {
    let json_val = serde_json::to_value(value)?;
    Ok(json_val)
}

/// Converts a serde_json::Value (e.g. pulled from Postgres JSONB) back into a Rust struct.
pub fn from_jsonb<T: for<'de> Deserialize<'de>>(json_val: JsonValue) -> Result<T, Box<dyn Error>> {
    let value = serde_json::from_value(json_val)?;
    Ok(value)
}
