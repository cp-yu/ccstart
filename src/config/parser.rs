use crate::error::AppResult;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ConfigCollection {
    pub claude: ClaudeConfig,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeConfig {
    pub providers: HashMap<String, Provider>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Provider {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(rename = "settingsConfig")]
    pub settings_config: Value,
}

/// 解析指定路径的 config.json
pub fn parse_config_file<P: AsRef<Path>>(path: P) -> AppResult<ConfigCollection> {
    let data = fs::read_to_string(path)?;
    let parsed: ConfigCollection = serde_json::from_str(&data)?;
    Ok(parsed)
}

