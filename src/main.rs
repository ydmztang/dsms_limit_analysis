use limit_analysis::db::common::OrderByOptions;
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    //limit_analysis::analysis::max_field_length::get_limit_coverage_by_config(&conn, OrderByOptions::Trending, 64_000);
    
    limit_analysis::analysis::max_field_length::get_desired_limit_by_config(
        &conn,
        OrderByOptions::Trending,
        0.01,
        0.99,
    );
}
