use rusqlite::{params, Connection, Row};

use crate::data_models::dataset_stats::DatasetStatsResponse;

use super::common::OrderByOptions;

pub fn initialize_dataset_stats_table(conn: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS dataset_stats (
        id TEXT,
        config TEXT,
        split TEXT,
        stats LONGBLOB,
        status_code BIGINT,
        error TEXT,
        PRIMARY KEY (id, config, split)
    );
";
    conn.execute(query, []).unwrap();
}

pub fn upsert_dataset_stats(
    conn: &Connection,
    id: &str,
    config: &str,
    split: &str,
    dataset_stats: Option<&DatasetStatsResponse>,
    status_code: reqwest::StatusCode,
    error: Option<&str>,
) {
    let query = "
    INSERT OR REPLACE INTO dataset_stats (id, config, split, stats, status_code, error)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6)
";

    let stats: Option<String> = dataset_stats.and_then(|stats| serde_json::to_string(stats).ok());
    conn.execute(
        query,
        params![id, config, split, stats, status_code.as_u16(), error],
    )
    .unwrap();
}

pub struct DatasetStatsInfoWrapper<'a> {
    statement: rusqlite::Statement<'a>,
    params: Vec<Box<dyn rusqlite::ToSql>>,
}

impl<'a> DatasetStatsInfoWrapper<'a> {
    pub fn new(statement: rusqlite::Statement<'a>, params: Vec<Box<dyn rusqlite::ToSql>>) -> Self {
        Self { statement, params }
    }

    pub fn get_iter(
        &'_ mut self,
    ) -> impl Iterator<Item = rusqlite::Result<(DatasetStatsId, DatasetStatsResponse)>> + '_ {
        let params = rusqlite::params_from_iter(self.params.iter());
        self.statement
            .query_map(params, |row| Ok(parse_dataset_info(row)))
            .unwrap()
    }
}

#[derive(Default, Clone)]
pub struct DatasetStatsId {
    pub dataset: String,
    pub config: String,
    pub split: String,
}

fn parse_dataset_info(row: &Row) -> (DatasetStatsId, DatasetStatsResponse) {
    let id: String = row.get("id").unwrap();
    let config: String = row.get("config").unwrap();
    let split: String = row.get("split").unwrap();
    let stats_str: String = row.get("stats").unwrap();
    (
        DatasetStatsId { dataset: id, config, split },
        serde_json::from_str(&stats_str).unwrap(),
    )
}

pub fn get_ordered_dataset_stats_info(
    conn: &Connection,
    order_by: OrderByOptions,
) -> DatasetStatsInfoWrapper<'_> {
    let stmt = conn
        .prepare(&format!(
            "
            SELECT * FROM datasets
            JOIN dataset_stats
            ON datasets.id=dataset_stats.id
            WHERE dataset_stats.status_code = 200
			ORDER BY {}, datasets.trending_score DESC, dataset_stats.id, dataset_stats.config
            ",
            order_by.as_string()
        ))
        .unwrap();
    DatasetStatsInfoWrapper::new(stmt, vec![])
}
