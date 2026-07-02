use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TemperatureData {
    pub id: String,
    pub temperature: f32,
}

pub fn get_env_or<T>(key: &str, default: T) -> T
where
    T: FromStr,
{
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
