use std::{cmp::max, collections::HashMap};

use crate::{
    analysis::constants::MAX_CARDINALITY,
    data_models::dataset_stats::StringLabelStats,
    db::{self, common::OrderByOptions, dataset_stats::DatasetStatsId},
};
use rusqlite::Connection;

pub fn get_label_cardinality_distribution(conn: &Connection, order_by: OrderByOptions, top: f64) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let target_count = (dataset_stats_count.configs as f64 * top) as usize;
    println!(
        "Total configs: {}, target configs to look up: {}",
        dataset_stats_count.configs, target_count
    );

    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut config_cardinality: usize = 0;
    let mut config_cardinality_counts = HashMap::<usize, usize>::new();
    let mut configs_has_labels = 0;
    let mut configs_visited = 0;
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            // if there is ever a label column then it counts
            if config_cardinality != 0 {
                configs_has_labels += 1;

                if config_cardinality > 50 {
                    println!("Found target: {:?}", last_stats_id);
                    return;
                }

                *config_cardinality_counts
                    .entry(config_cardinality)
                    .or_insert(0) += 1;
            }

            // progress tracker
            configs_visited += 1;
            if configs_visited > target_count {
                break;
            }
            if configs_visited % 100 == 0 {
                print!(
                    "Percentage visited: {}\r",
                    100.0 * configs_visited as f64 / target_count as f64
                );
            }

            // reset status
            last_stats_id = dataset_stats_id.clone();
            config_cardinality = 0;
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "string_label"
                && column.column_name.to_lowercase().contains("label")
            {
                let stats = serde_json::from_value::<StringLabelStats>(column.column_statistics);
                if let Err(err) = stats {
                    println!("dataset: {:?}, error: {:?}", dataset_stats_id, err);
                } else if let Ok(stats) = stats {
                    config_cardinality = max(config_cardinality, stats.n_unique as usize);
                }
            }
        }
    }
    println!(
        "\nConfigs visited: {}, configs has labels {}",
        configs_visited, configs_has_labels
    );

    let mut covered_configs = 0;
    println!("Cardinality,Coverage");
    for i in 0..=MAX_CARDINALITY {
        covered_configs += config_cardinality_counts.get(&i).unwrap_or(&0);
        println!(
            "{},{}",
            i,
            100.0 * covered_configs as f64 / configs_has_labels as f64
        );
    }
}
