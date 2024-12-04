use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DatasetInfoResponse {
    pub dataset_info: HashMap<String, DatasetInfo>,
    pub pending: Vec<Value>,
    pub failed: Vec<Value>,
    pub partial: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DatasetInfo {
    pub description: String,
    pub citation: String,
    pub homepage: String,
    pub license: String,
    pub features: Value,
    pub builder_name: String,
    pub dataset_name: String,
    pub config_name: String,
    pub version: Version,
    pub splits: HashMap<String, SplitInfo>,
    pub download_checksums: Option<Value>,
    pub download_size: i64,
    pub dataset_size: i64,
    pub size_in_bytes: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Version {
    pub version_str: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SplitInfo {
    pub name: String,
    pub num_bytes: i64,
    pub num_examples: i64,
    pub dataset_name: Option<String>,
}
