use crate::db;
use rusqlite::Connection;

// Report the number every {GRANULARITY} percentage of datasets
const GRANULARITY: f32 = 0.01;

pub fn get_dataset_row_limit_coverage_by_dataset(conn: &Connection, order_by: &str, limit: i64) {
    let dataset_count = db::dataset_info::get_datasets_count(conn);
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

pub fn get_dataset_row_limit_coverage_by_config(conn: &Connection, order_by: &str, limit: i64) {
    let dataset_count = db::dataset_info::get_datasets_count(conn);
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
