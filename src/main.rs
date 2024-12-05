use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./analysis.db").unwrap();
    
    // limit_analysis::scrape::common::scrape_all_data(&conn);
    
    // downloads, likes, trending_score
    limit_analysis::analysis::dataset_rows::get_desired_limit_by_config(&conn, "trending_score", 0.01, 0.9);
}


