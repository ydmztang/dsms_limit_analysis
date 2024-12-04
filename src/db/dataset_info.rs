use rusqlite::{params, Connection};

use crate::data_models::dataset_info::DatasetInfoResponse;

pub fn initialize_dataset_info_table(conn: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS dataset_info (
        _id TEXT PRIMARY KEY,
        id TEXT,
        info LONGBLOB,
        status_code BIGINT,
        error TEXT
    );
";
    conn.execute(query, []).unwrap();
}

pub fn upsert_dataset_info(
    conn: &Connection,
    _id: &str,
    id: &str,
    dataset_info: Option<&DatasetInfoResponse>,
    status_code: reqwest::StatusCode,
    error: Option<&str>,
) {
    let query = "
    INSERT OR REPLACE INTO dataset_info (_id, id, info, status_code, error)
    VALUES (?1, ?2, ?3, ?4, ?5)
";

    let info: Option<String> = dataset_info.and_then(|info| serde_json::to_string(info).ok());
    conn.execute(query, params![_id, id, info, status_code.as_u16(), error])
        .unwrap();
}
