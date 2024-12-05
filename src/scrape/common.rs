use rusqlite::Connection;

use super::{dataset_info, dataset_stats, datasets};

pub async fn scrape_all_data(conn: &Connection) {
    datasets::fetch_and_save_all_datasets(conn).await;
    dataset_info::fetch_and_save_all_datasets_info(conn).await;
    dataset_stats::fetch_and_save_all_datasets_stats(conn).await;
}