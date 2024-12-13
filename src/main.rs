use limit_analysis::db::common::OrderByOptions;
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    limit_analysis::analysis::task_popularity::get_task_popularity(
        &conn,
        OrderByOptions::Trending,
        1.0,
    );
}
