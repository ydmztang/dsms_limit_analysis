use std::{cmp::max, collections::HashMap};

use crate::{
    data_models::dataset_stats::{ImageStats}, db::{self, common::OrderByOptions, dataset_stats::DatasetStatsId}
};
use rusqlite::Connection;

// Get the max image size distribution, bucket size is in pixels
pub fn get_image_size_distribution(
    conn: &Connection,
    order_by: OrderByOptions,
    top: f64,
    bucket_size: usize,
) {
    let dataset_stats_count = db::total_dataset_stats_count::get_dataset_stats_count(conn);
    let target_count = (dataset_stats_count.configs as f64 * top) as usize;
    println!(
        "Total configs: {}, target configs (that has image columns) to look up: {}",
        dataset_stats_count.configs, target_count
    );

    let mut last_stats_id: DatasetStatsId = DatasetStatsId::default();
    let mut max_image_width = 0;
    let mut size_buckets = HashMap::<usize, usize>::new();
    let mut total_image_configs = 0;
    for dataset_stats in db::dataset_stats::get_ordered_dataset_stats_info(conn, order_by).get_iter() {
        let (dataset_stats_id, dataset_stats_response) = dataset_stats.unwrap();
        
        // Encountered a new config
        if dataset_stats_id.dataset != last_stats_id.dataset || dataset_stats_id.config != last_stats_id.config {     
            // update count in the bucket
            if max_image_width != 0 {
                total_image_configs += 1; 

                let bucket_index = max_image_width / bucket_size;
                *size_buckets.entry(bucket_index).or_insert(0) += 1;

                if total_image_configs > target_count {
                    break;
                }

                if total_image_configs % 100 == 0 {
                    println!("Configs visited: {}\r", total_image_configs);
                }
            }

            max_image_width = 0;
            last_stats_id = dataset_stats_id.clone();
        }

        for column in dataset_stats_response.statistics {
            if column.column_type == "image" {
                let stats  = serde_json::from_value::<ImageStats>(column.column_statistics);
                if let Err(err) = stats {
                    println!("dataset: {:?}, error: {:?}", dataset_stats_id, err);
                } else if let Ok(stats) = stats {
                    //let stats: ImageStats = serde_json::from_value(column.column_statistics).unwrap();
                    if stats.max.is_some() {
                        max_image_width = max(max_image_width, stats.max.unwrap() as usize);
                    }
                }
            }
        }
    }
    println!();

    let mut keys: Vec<usize> = size_buckets.keys().copied().collect::<Vec<usize>>();
    keys.sort();
    let max_bucket_index = *keys.last().unwrap();
    let mut covered_images = 0;
    
    println!("Image Width,Count");
    for i in 0..=max_bucket_index {
        covered_images += size_buckets.get(&i).unwrap_or(&0);
        println!("{},{}", i * bucket_size, 100.0 * covered_images as f64 / total_image_configs as f64);
    }
}
