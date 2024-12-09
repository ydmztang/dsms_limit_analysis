use crate::{
    analysis::constants::GRANULARITY,
    db::{self, common::OrderByOptions},
};
use rusqlite::Connection;

// Check how much datasets our current limit can cover
pub fn get_dataset_row_limit_coverage_by_dataset(
    conn: &Connection,
    order_by: OrderByOptions,
    limit: i64,
) {
    let dataset_count = db::total_dataset_count::get_dataset_count(conn);
    let report_interval = (dataset_count.datasets as f32 * GRANULARITY) as i32;
    println!(
        "Report granularity is {}, reporta interval is every {} datasets",
        GRANULARITY, report_interval
    );

    let mut visited_count = 0;
    let mut cover_count = 0;
    for dataset_info in db::dataset_info::get_ordered_dataset_info(conn, order_by).get_iter() {
        visited_count += 1;

        let (_, dataset_info_response) = dataset_info.unwrap();
        let mut dataset_rows = 0;
        for (_, config_info) in dataset_info_response.dataset_info {
            for (_, split_info) in config_info.splits {
                dataset_rows += split_info.num_examples;
            }
        }

        if limit > dataset_rows {
            cover_count += 1;
        }

        if visited_count % report_interval == 0 {
            println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
        }
    }

    // The last segment of data
    if visited_count % report_interval != 0 {
        println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
    }
}

pub fn get_dataset_row_limit_coverage_by_config(
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

            let mut config_rows = 0;
            for (_, split_info) in config_info.splits {
                config_rows += split_info.num_examples;
            }
            if limit > config_rows {
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

// Get the limit in order to cover top N% datasets with M% coverage
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
    let mut config_row_counts: Vec<i64> = vec![];
    for dataset_info in db::dataset_info::get_ordered_dataset_info(conn, order_by).get_iter() {
        let (_, dataset_info_response) = dataset_info.unwrap();
        for (_, config_info) in dataset_info_response.dataset_info {
            let mut config_rows = 0;
            for (_, split_info) in config_info.splits {
                config_rows += split_info.num_examples;
            }
            config_row_counts.push(config_rows);
            visited_count += 1;
            if visited_count >= top_count {
                config_row_counts.sort();
                println!("The desired limit is {}", config_row_counts[target_count]);
                return;
            }
        }
    }
}
