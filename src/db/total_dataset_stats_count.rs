use rusqlite::{params, Connection};

pub struct DatasetStatsCount {
    pub datasets: i64,
    pub configs: i64,
    pub splits: i64,
}

pub fn get_dataset_stats_count(conn: &Connection) -> DatasetStatsCount {
    let mut stmt = conn.prepare("SELECT * FROM total_dataset_stats_count").unwrap();

    stmt.query_row([], |row| {
        Ok(DatasetStatsCount {
            datasets: row.get("datasets").unwrap(),
            configs: row.get("configs").unwrap(),
            splits: row.get("splits").unwrap(),
        })
    })
    .unwrap()
}

// Only count the stats that are successful
pub fn calculate_and_update_total_dataset_stats_count(conn: &Connection) {
    initialize_total_dataset_count_table(conn);
    let dataset_count = calculate_dataset_stats_count(conn);
    let query = "
    INSERT OR REPLACE INTO total_dataset_stats_count (datasets, configs, splits)
    VALUES (?1, ?2, ?3)
";
    conn.execute(
        query,
        params![
            dataset_count.datasets,
            dataset_count.configs,
            dataset_count.splits
        ],
    )
    .unwrap();
}

fn initialize_total_dataset_count_table(conn: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS total_dataset_stats_count (
        datasets BIGINT,
        configs BIGINT,
        splits BIGINT,
        PRIMARY KEY (datasets, configs, splits)
    );
";
    conn.execute(query, []).unwrap();
}

fn calculate_dataset_stats_count(conn: &Connection) -> DatasetStatsCount {
    println!("Getting total dataset stats count. This may take a while...");
    let mut splits_count_stmt = conn.prepare("SELECT COUNT(*) FROM dataset_stats where dataset_stats.status_code = 200").unwrap();
    let splits_count: i64 = splits_count_stmt.query_row([], |row| row.get(0)).unwrap();    

    let mut configs_count_stmt = conn.prepare("
    SELECT COUNT(*) FROM 
	    (SELECT DISTINCT dataset_stats.id, dataset_stats.config FROM datasets
            JOIN dataset_stats
            ON datasets.id=dataset_stats.id
            WHERE dataset_stats.status_code = 200)
    ").unwrap();
    let configs_count: i64 = configs_count_stmt.query_row([], |row| row.get(0)).unwrap();

    let mut datasets_count_stmt = conn.prepare("
    SELECT COUNT(*) FROM 
	    (SELECT DISTINCT dataset_stats.id FROM datasets
            JOIN dataset_stats
            ON datasets.id=dataset_stats.id
            WHERE dataset_stats.status_code = 200)
    ").unwrap();
    let datasets_count: i64 = datasets_count_stmt.query_row([], |row| row.get(0)).unwrap();

    DatasetStatsCount {
        datasets: datasets_count,
        configs: configs_count,
        splits: splits_count,
    }
}
