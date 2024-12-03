
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::open("./analysis.db").unwrap();
    
    // limit_analysis::db::datasets::initialize_datasets_table(&connection);
    limit_analysis::scrape::datasets::fetch_and_save_all_datasets(&connection).await;
    
}
