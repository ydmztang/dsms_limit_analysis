use std::cmp::max;

use crate::{
    analysis::constants::GRANULARITY,
    data_models::dataset_stats::StringTextStats,
    db::{self, common::OrderByOptions, dataset_stats::DatasetStatsId},
};
use rusqlite::Connection;

pub fn get_limit_coverage_by_config(conn: &Connection, order_by: OrderByOptions, limit: usize) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let report_interval = (dataset_stats_count.configs as f32 * GRANULARITY) as i32;
    println!(
        "Report granularity is {}, reporta interval is every {} configs",
        GRANULARITY, report_interval
    );

    let mut visited_count = 0;
    let mut cover_count = 0;
    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut max_text_length = 0;
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            // calculate coverage
            if visited_count > 0 {
                if limit > max_text_length {
                    cover_count += 1;
                }

                if visited_count % report_interval == 0 {
                    println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
                }
            }

            // initialize variables for new config
            max_text_length = 0;
            last_stats_id = dataset_stats_id.clone();
            visited_count += 1;
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "string_text" {
                let stats: StringTextStats =
                    serde_json::from_value(column.column_statistics).unwrap();
                max_text_length = max(max_text_length, stats.max as usize);
            }
        }
    }

    // The last segment of data
    visited_count += 1;
    if visited_count % report_interval != 0 {
        println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
    }
}

pub fn get_desired_limit_by_config(
    conn: &Connection,
    order_by: OrderByOptions,
    top: f64,
    desired_coverage: f64,
) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let top_count = (dataset_stats_count.configs as f64 * top) as usize;
    let target_count = (top_count as f64 * desired_coverage) as usize;
    println!(
        "Total configs: {}, top configs to look up: {}, desired configs to cover: {}",
        dataset_stats_count.configs, top_count, target_count
    );

    let mut visited_count = 0;
    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut max_text_length = 0;
    let mut max_text_lengths: Vec<usize> = vec![];
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            max_text_lengths.push(max_text_length);

            max_text_length = 0;
            last_stats_id = dataset_stats_id.clone();
            visited_count += 1;

            if visited_count >= top_count {
                // Sort in ascending order using `partial_cmp`
                max_text_lengths.sort();
                println!("The desired limit is {}", max_text_lengths[target_count]);
                return;
            }
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "string_text" {
                let stats: StringTextStats =
                    serde_json::from_value(column.column_statistics).unwrap();
                max_text_length = max(max_text_length, stats.max as usize);
            }
        }
    }
}
