use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// doc: https://huggingface.co/docs/dataset-viewer/en/statistics#list

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DatasetStatsResponse {
    pub num_examples: i64,
    pub statistics: Vec<ColumnStats>,
    pub partial: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ColumnStats {
    pub column_name: String,
    pub column_type: String,
    pub column_statistics: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Statistics {
    #[default]
    None,
    ClassLabel(ClassLabelStats),
    Float(FloatStats),
    Int(IntStats),
    Bool(BoolStats),
    StringLabel(StringLabelStats),
    StringText(StringTextStats),
    List(ListStats),
    Audio(AudioStats),
    Image(ImageStats),
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Histogram {
    pub hist: Vec<i64>,
    pub bin_edges: Vec<f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClassLabelStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub no_label_count: i64,
    pub no_label_proportion: f64,
    pub n_unique: i64,
    pub frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FloatStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IntStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: i64,
    pub max: i64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BoolStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StringLabelStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub n_unique: i64,
    pub frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StringTextStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: i64,
    pub max: i64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ListStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: i64,
    pub max: i64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageStats {
    pub nan_count: i64,
    pub nan_proportion: f64,
    pub min: i64,
    pub max: i64,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub histogram: Histogram,
}
