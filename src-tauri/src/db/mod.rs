pub mod pool;

use serde::{de::DeserializeOwned, Serialize};

/// Convert a Rust enum (with serde rename_all = "snake_case") to its DB string representation.
pub fn enum_to_str<T: Serialize>(val: &T) -> String {
    serde_json::to_value(val)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

/// Parse a DB string into a Rust enum (with serde rename_all = "snake_case").
pub fn parse_enum<T: DeserializeOwned>(s: &str) -> T {
    serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap()
}

/// Convert a Vec of enums to Vec<String> for DB TEXT[] columns.
pub fn enum_vec_to_strs<T: Serialize>(v: &[T]) -> Vec<String> {
    v.iter().map(|e| enum_to_str(e)).collect()
}

/// Parse a Vec<String> from DB TEXT[] into a Vec of enums.
pub fn parse_enum_vec<T: DeserializeOwned>(v: &[String]) -> Vec<T> {
    v.iter().map(|s| parse_enum(s)).collect()
}
