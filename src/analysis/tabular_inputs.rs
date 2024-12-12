use std::collections::HashSet;

use crate::db::{self, common::OrderByOptions, dataset_stats::DatasetStatsId};
use rusqlite::Connection;

pub fn get_tabular_inputs_details(conn: &Connection, order_by: OrderByOptions, top: f64) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let target_count = (dataset_stats_count.configs as f64 * top) as usize;

    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut labels = HashSet::<String>::new();
    let mut non_labels = HashSet::<String>::new();
    let mut configs_visited = 0;
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            if !labels.is_empty() {
                let label_count = labels.len();
                let non_label_count = non_labels.len();
                let label_names = labels.into_iter().collect::<Vec<String>>().join("; ");
                let non_label_names = non_labels.into_iter().collect::<Vec<String>>().join("; ");

                // dataset_id, config, label count, non-label count, ratio, label names, non label names
                println!(
                    "{},{},{},{},{},{},{}",
                    last_stats_id.dataset,
                    last_stats_id.config,
                    label_count,
                    non_label_count,
                    label_count as f64 / non_label_count as f64,
                    label_names,
                    non_label_names,
                );
            }

            // progress tracker
            configs_visited += 1;
            if configs_visited > target_count {
                break;
            }

            // reset status
            last_stats_id = dataset_stats_id.clone();
            labels = HashSet::new();
            non_labels = HashSet::new();
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "string_label"
                && column.column_name.to_lowercase().contains("label")
            {
                labels.insert(column.column_name.clone());
            } else {
                non_labels.insert(column.column_name.clone());
            }
        }
    }
}
