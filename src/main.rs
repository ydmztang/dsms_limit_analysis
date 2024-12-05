use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::open("./analysis.db").unwrap();
    // limit_analysis::scrape::datasets::fetch_and_save_all_datasets(&connection).await;
    // limit_analysis::scrape::dataset_info::fetch_and_save_all_datasets_info(&connection).await;
    // limit_analysis::scrape::dataset_stats::fetch_and_save_all_datasets_stats(&connection).await;

    // limit_analysis::analysis::dataset_rows::get_dataset_row_limit_coverage(&connection, "downloads", 100_000);

    let mut info = limit_analysis::db::dataset_info::get_dataset_info(&connection, "cis-lmu/GlotCC-V1");
    for data in info.get_iter() {
        println!("dataset id: {}", data.unwrap().0);
    }
}
