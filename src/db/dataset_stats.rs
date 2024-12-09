use rusqlite::{params, Connection};

use crate::data_models::dataset_stats::DatasetStatsResponse;


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

// pub fn get_ordered_dataset_stats_info(
//     conn: &Connection,
//     order_by: OrderByOptions,
// ) -> DatasetInfoWrapper<'_> {
//     let stmt = conn
//         .prepare(&format!(
//             "
//             SELECT * FROM datasets
//             JOIN dataset_info
//                 ON datasets._id=dataset_info._id
//             WHERE dataset_info.status_code = 200
//             ORDER BY {}
//             DESC
//             ",
//             order_by.as_string()
//         ))
//         .unwrap();
//     DatasetInfoWrapper::new(stmt, vec![])
// }