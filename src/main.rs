use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    limit_analysis::analysis::dataset_rows::get_desired_limit_by_config(
        &conn,
        limit_analysis::db::dataset_info::OrderByOptions::Trending,
        0.01,
        0.9,
    );
}
