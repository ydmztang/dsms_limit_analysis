use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    //limit_analysis::analysis::max_columns::get_max_columns_limit_coverage_by_config(&conn, OrderByOptions::Downloads, 20);
    
    limit_analysis::analysis::max_columns::get_desired_limit_by_config(
        &conn,
        limit_analysis::db::common::OrderByOptions::Downloads,
        0.01,
        0.99,
    );
}
