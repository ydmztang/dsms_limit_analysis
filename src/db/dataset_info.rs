
use rusqlite::{params, Connection, Row};

use crate::data_models::dataset_info::DatasetInfoResponse;

use super::common::OrderByOptions;

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
    params: Vec<Box<dyn rusqlite::ToSql>>,
}

impl<'a> DatasetInfoWrapper<'a> {
    pub fn new(statement: rusqlite::Statement<'a>, params: Vec<Box<dyn rusqlite::ToSql>>) -> Self {
        Self { statement, params }
    }

    pub fn get_iter(
        &'_ mut self,
    ) -> impl Iterator<Item = rusqlite::Result<(String, DatasetInfoResponse)>> + '_ {
        let params = rusqlite::params_from_iter(self.params.iter());
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
    DatasetInfoWrapper {
        statement: stmt,
        params: vec![],
    }
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
    DatasetInfoWrapper {
        statement: stmt,
        params: vec![],
    }
}

pub fn get_dataset_info<'a>(conn: &'a Connection, dataset_id: &str) -> DatasetInfoWrapper<'a> {
    let stmt = conn
        .prepare("SELECT * FROM dataset_info WHERE id = ?1")
        .unwrap();

    DatasetInfoWrapper {
        statement: stmt,
        params: vec![Box::new(dataset_id.to_string())],
    }
}

pub fn get_ordered_dataset_info(
    conn: &Connection,
    order_by: OrderByOptions,
) -> DatasetInfoWrapper<'_> {
    let stmt = conn
        .prepare(&format!(
            "
            SELECT * FROM datasets
            JOIN dataset_info
                ON datasets._id=dataset_info._id
            WHERE dataset_info.status_code = 200
            ORDER BY {}
            DESC
            ",
            order_by.as_string()
        ))
        .unwrap();
    DatasetInfoWrapper::new(stmt, vec![])
}
