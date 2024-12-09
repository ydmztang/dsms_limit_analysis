use crate::db::{total_dataset_count::calculate_and_update_total_dataset_count, total_dataset_stats_count::calculate_and_update_total_dataset_stats_count};
use rusqlite::Connection;

use super::{dataset_info, dataset_stats, datasets};

pub async fn scrape_all_data(conn: &Connection) {
    datasets::fetch_and_save_all_datasets(conn).await;
    dataset_info::fetch_and_save_all_datasets_info(conn).await;
    dataset_stats::fetch_and_save_all_datasets_stats(conn).await;
    calculate_and_update_total_dataset_count(conn);
    calculate_and_update_total_dataset_stats_count(conn);
}
