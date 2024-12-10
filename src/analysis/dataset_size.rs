use crate::{
    analysis::constants::GRANULARITY,
    db::{self, common::OrderByOptions},
};
use rusqlite::Connection;

// Get dataset size limit (in GB) coverage
pub fn get_limit_coverage_by_config(
    conn: &Connection,
    order_by: OrderByOptions,
    limit: i64,
) {
    let dataset_count = db::total_dataset_count::get_dataset_count(conn);
    let report_interval = (dataset_count.configs as f32 * GRANULARITY) as i32;
    println!(
        "Report granularity is {}, reporta interval is every {} configs",
        GRANULARITY, report_interval
    );

    let mut visited_count = 0;
    let mut cover_count = 0;
    for dataset_info in db::dataset_info::get_ordered_dataset_info(conn, order_by).get_iter() {
        let (_, dataset_info_response) = dataset_info.unwrap();
        for (_, config_info) in dataset_info_response.dataset_info {
            visited_count += 1;

            // size in dataset info are in bytes, convert to GB
            let config_size = config_info.dataset_size / 1_000_000_000;
            if limit > config_size {
                cover_count += 1;
            }

            if visited_count % report_interval == 0 {
                println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
            }
        }
    }

    // The last segment of data
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
    let dataset_count = db::total_dataset_count::get_dataset_count(conn);
    let top_count = (dataset_count.configs as f64 * top) as usize;
    let target_count = (top_count as f64 * desired_coverage) as usize;
    println!(
        "Total configs: {}, top configs to look up: {}, desired configs to cover: {}",
        dataset_count.configs, top_count, target_count
    );

    let mut visited_count = 0;
    let mut config_sizes: Vec<f64> = vec![];
    for dataset_info in db::dataset_info::get_ordered_dataset_info(conn, order_by).get_iter() {
        let (_, dataset_info_response) = dataset_info.unwrap();
        for (_, config_info) in dataset_info_response.dataset_info {
            config_sizes.push(config_info.dataset_size as f64 / 1_000_000_000_f64);
            visited_count += 1;
            if visited_count >= top_count {
                // Sort in ascending order using `partial_cmp`
                config_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                println!("The desired limit is {}", config_sizes[target_count]);
                return;
            }
        }
    }
}
