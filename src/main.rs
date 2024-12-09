use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    // limit_analysis::analysis::dataset_size::get_dataset_size_limit_coverage_by_config(&conn, OrderByOptions::Downloads, 128);
    // limit_analysis::analysis::dataset_size::get_desired_limit_by_config(
    //     &conn,
    //     limit_analysis::db::common::OrderByOptions::Trending,
    //     0.01,
    //     0.9,
    // );

    limit_analysis::db::total_dataset_stats_count::get_dataset_stats_count(&conn);
}
