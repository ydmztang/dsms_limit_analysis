use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::open("./analysis.db").unwrap();
    // limit_analysis::scrape::datasets::fetch_and_save_all_datasets(&connection).await;
    // limit_analysis::scrape::dataset_info::fetch_and_save_all_datasets_info(&connection).await;

    limit_analysis::scrape::dataset_stats::fetch_and_save_all_datasets_stats(&connection).await;
}
