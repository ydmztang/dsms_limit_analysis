use limit_analysis::db::common::OrderByOptions;
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();

    // limit_analysis::scrape::common::scrape_all_data(&conn);

    limit_analysis::analysis::label_min_examples::get_label_min_examples_distribution(
        &conn,
        OrderByOptions::Trending,
        0.01,
    );
}
