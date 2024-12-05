use rusqlite::{params, Connection, Row};

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

pub struct DatasetInfoWrapper<'a> {
    statement: rusqlite::Statement<'a>,
}

impl DatasetInfoWrapper<'_> {
    pub fn get_iter<P>(
        &'_ mut self,
        params: P,
    ) -> impl Iterator<Item = rusqlite::Result<(String, DatasetInfoResponse)>> + '_
    where
        P: rusqlite::Params,
    {
        self.statement
            .query_map(params, |row| Ok(parse_dataset_info(row)))
            .unwrap()
    }
}

fn parse_dataset_info(row: &Row) -> (String, DatasetInfoResponse) {
    let id: String = row.get("id").unwrap();
    let info_str: String = row.get("info").unwrap();
    (id, serde_json::from_str(&info_str).unwrap())
}

pub fn list_all_datasets_has_info(conn: &Connection) -> DatasetInfoWrapper {
    let stmt = conn
        .prepare("SELECT * FROM dataset_info where status_code = 200")
        .unwrap();
    DatasetInfoWrapper { statement: stmt }
}

pub fn list_datasets_has_info_but_no_stats(conn: &Connection) -> DatasetInfoWrapper {
    let stmt = conn
        .prepare(
            "
        SELECT * FROM dataset_info  
        LEFT JOIN dataset_stats
            ON dataset_info.id = dataset_stats.id
        WHERE dataset_stats.id is NULL
            AND dataset_info.status_code = 200
",
        )
        .unwrap();
    DatasetInfoWrapper { statement: stmt }
}

pub fn get_dataset_info(conn: &Connection) -> DatasetInfoWrapper<'_> {
    let stmt = conn
        .prepare("SELECT * FROM dataset_info where id = ?1")
        .unwrap();

    DatasetInfoWrapper { statement: stmt }
}
