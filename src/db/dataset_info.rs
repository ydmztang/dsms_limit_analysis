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

pub struct DatasetCount {
    pub datasets: i64,
    pub configs: i64,
    pub splits: i64,
}

pub fn get_datasets_count(conn: &Connection) -> DatasetCount {
    println!("Getting count. This may take a while...");
    let mut datasets = list_all_datasets_has_info(conn);
    let mut dataset_count = 0;
    let mut config_count = 0;
    let mut split_count = 0;
    for dataset in datasets.get_iter() {
        dataset_count += 1;
        let (_, dataset_info_response) = dataset.unwrap();
        for (_, config_info) in dataset_info_response.dataset_info {
            config_count += 1;
            for (_, _) in config_info.splits {
                split_count += 1;
            }
        }

        if dataset_count % 1000 == 0 {
            print!("\rVisited {}K datasets", dataset_count / 1000);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }
    println!();

    DatasetCount {
        datasets: dataset_count,
        configs: config_count,
        splits: split_count,
    }
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

pub enum OrderByOptions {
    Trending,
    Likes,
    Downloads,
}

pub fn get_ordered_dataset_info(
    conn: &Connection,
    order_by: OrderByOptions,
) -> DatasetInfoWrapper<'_> {
    let order_by_str = match order_by {
        OrderByOptions::Trending => "trending_score",
        OrderByOptions::Likes => "likes",
        OrderByOptions::Downloads => "downloads",
    };

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
            order_by_str
        ))
        .unwrap();
    DatasetInfoWrapper::new(stmt, vec![])
}
