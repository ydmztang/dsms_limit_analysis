use std::collections::HashSet;

use crate::{
    analysis::constants::GRANULARITY,
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
    let mut column_names = HashSet::<String>::new();
    let mut visited_configs = HashSet::<String>::new();
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        let mut is_new_config = false;
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            // calculate coverage
            if visited_count > 0 {
                if limit > column_names.len() {
                    cover_count += 1;
                }

                if visited_count % report_interval == 0 {
                    println!("{}", cover_count as f64 / visited_count as f64 * 100_f64);
                }
            }

            // initialize variables for new config
            column_names = HashSet::<String>::new();
            last_stats_id = dataset_stats_id.clone();
            visited_count += 1;

            // some error check
            is_new_config = true;
            let config_id = format!("{}_{}", dataset_stats_id.dataset, dataset_stats_id.config);
            if visited_configs.contains(&config_id) {
                panic!("???????? Encounter the config twice in a non consequtive way. Dataset: {}, Config: {}", &dataset_stats_id.dataset, &dataset_stats_id.config);
            }
            visited_configs.insert(config_id);
        }

        for column in dataset_stats_response.statistics {
            if !is_new_config && !column_names.contains(&column.column_name) {
                // The columns may be different between splits. E.g., Dataset: Asimok/KGLQA-KnowledgeBank-QuALITY, Config: caption. The "dev" and "test" splits has different columns
                // panic!("???????? Found new column name in another split of a config. Dataset: {}, Config: {}, Split: {}", &dataset_stats_id.dataset, &dataset_stats_id.config, &dataset_stats_id.split);
            }
            column_names.insert(column.column_name);
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
    let mut column_names = HashSet::<String>::new();
    let mut columns_counts: Vec<usize> = vec![];
    for dataset_stats in
        db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter()
    {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();

        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset
            || dataset_stats_id.config != last_stats_id.config
        {
            columns_counts.push(column_names.len());

            column_names = HashSet::<String>::new();
            last_stats_id = dataset_stats_id.clone();
            visited_count += 1;

            if visited_count >= top_count {
                // Sort in ascending order using `partial_cmp`
                columns_counts.sort();
                println!("The desired limit is {}", columns_counts[target_count]);
                return;
            }
        }

        for column in dataset_stats_response.statistics {
            column_names.insert(column.column_name);
        }
    }
}
