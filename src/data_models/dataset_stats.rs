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

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
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
    hist: Vec<i64>,
    bin_edges: Vec<f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClassLabelStats {
    nan_count: i64,
    nan_proportion: f64,
    no_label_count: i64,
    no_label_proportion: f64,
    n_unique: i64,
    frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FloatStats {
    nan_count: i64,
    nan_proportion: f64,
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IntStats {
    nan_count: i64,
    nan_proportion: f64,
    min: i64,
    max: i64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BoolStats {
    nan_count: i64,
    nan_proportion: f64,
    frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StringLabelStats {
    nan_count: i64,
    nan_proportion: f64,
    n_unique: i64,
    frequencies: HashMap<String, i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StringTextStats {
    nan_count: i64,
    nan_proportion: f64,
    min: i64,
    max: i64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ListStats {
    nan_count: i64,
    nan_proportion: f64,
    min: i64,
    max: i64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioStats {
    nan_count: i64,
    nan_proportion: f64,
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageStats {
    nan_count: i64,
    nan_proportion: f64,
    min: i64,
    max: i64,
    mean: f64,
    median: f64,
    std: f64,
    histogram: Histogram,
}
