use std::{cmp::min, collections::HashMap};

use crate::{
    data_models::dataset_stats::StringLabelStats,
    db::{self, common::OrderByOptions, dataset_stats::DatasetStatsId},
};
use rusqlite::Connection;

const EXAMPLES_UPPER_LIMIT: usize = 100;

pub fn get_label_min_examples_distribution(conn: &Connection, order_by: OrderByOptions, top: f64) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let target_count = (dataset_stats_count.configs as f64 * top) as usize;
    println!(
        "Total configs: {}, target configs to look up: {}",
        dataset_stats_count.configs, target_count
    );

    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut min_examples_counts = HashMap::<usize, usize>::new();
    let mut median_examples_counts = HashMap::<usize, usize>::new();
    // A map of <Column Name, <Class Name, Class Count>>
    let mut label_frequencies: HashMap<String, HashMap<String, usize>> = HashMap::new();
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
            if !label_frequencies.is_empty() {
                configs_has_labels += 1;

                let (min_examples, median_examples) =
                    get_label_stats(&label_frequencies, last_stats_id);
                let min_examples = min(EXAMPLES_UPPER_LIMIT, min_examples);
                let median_examples = min(EXAMPLES_UPPER_LIMIT, median_examples);

                *min_examples_counts.entry(min_examples).or_insert(0) += 1;
                *median_examples_counts.entry(median_examples).or_insert(0) += 1;
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
            label_frequencies = HashMap::new();
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "string_label"
                && column.column_name.to_lowercase().contains("label")
            {
                let stats = serde_json::from_value::<StringLabelStats>(column.column_statistics);
                if let Err(err) = stats {
                    println!("dataset: {:?}, error: {:?}", dataset_stats_id, err);
                } else if let Ok(stats) = stats {
                    if stats.n_unique > 0 {
                        let current_label_frequency = label_frequencies
                            .entry(column.column_name)
                            .or_default();

                        for (class, frequency) in stats.frequencies {
                            *current_label_frequency.entry(class).or_insert(0) +=
                                frequency as usize;
                        }
                    }
                }
            }
        }
    }
    println!(
        "\nConfigs visited: {}, configs has labels {}",
        configs_visited, configs_has_labels
    );

    // println!("Min labels count: {:?}", min_examples_counts);
    // println!("Median labels count: {:?}", median_examples_counts);
    println!("-----------------------------------------------------------------------");
    println!("Minimum Examples Limit,Coverage Min Example,Coverage Meadiam Example");
    let mut covered_configs_by_min_examples = 0;
    let mut covered_configs_by_median_examples = 0;
    for i in (0..=EXAMPLES_UPPER_LIMIT).rev() {
        covered_configs_by_min_examples += min_examples_counts.get(&i).unwrap_or(&0);
        covered_configs_by_median_examples += median_examples_counts.get(&i).unwrap_or(&0);
        println!(
            "{},{},{}",
            i,
            100.0 * covered_configs_by_min_examples as f64 / configs_has_labels as f64,
            100.0 * covered_configs_by_median_examples as f64 / configs_has_labels as f64,
        );
    }
}

// calculate the min and median examples
fn get_label_stats(
    label_frequencies: &HashMap<String, HashMap<String, usize>>,
    _dataset_stats_id: DatasetStatsId,
) -> (usize, usize) {
    let mut counts_across_all_labels: Vec<usize> = vec![];
    for class_counts in label_frequencies.values() {
        for count in class_counts.values() {
            counts_across_all_labels.push(*count);
        }
    }

    counts_across_all_labels.sort();
    (
        counts_across_all_labels[0],
        counts_across_all_labels[counts_across_all_labels.len() / 2],
    )
}
