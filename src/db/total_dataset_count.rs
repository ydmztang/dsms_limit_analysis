use rusqlite::{params, Connection};

pub struct DatasetCount {
    pub datasets: i64,
    pub configs: i64,
    pub splits: i64,
}

pub fn get_dataset_count(conn: &Connection) -> DatasetCount {
    let mut stmt = conn.prepare("SELECT * FROM total_dataset_count").unwrap();

    stmt.query_row([], |row| {
        Ok(DatasetCount {
            datasets: row.get("datasets").unwrap(),
            configs: row.get("configs").unwrap(),
            splits: row.get("splits").unwrap(),
        })
    })
    .unwrap()
}

pub fn calculate_and_update_total_dataset_count(conn: &Connection) {
    initialize_total_dataset_count_table(conn);
    let dataset_count = calculate_datasets_count(conn);
    let query = "
    INSERT OR REPLACE INTO total_dataset_count (datasets, configs, splits)
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
    CREATE TABLE IF NOT EXISTS total_dataset_count (
        datasets BIGINT,
        configs BIGINT,
        splits BIGINT,
        PRIMARY KEY (datasets, configs, splits)
    );
";
    conn.execute(query, []).unwrap();
}

fn calculate_datasets_count(conn: &Connection) -> DatasetCount {
    println!("Getting total datasets count. This may take a while...");
    let mut datasets = crate::db::dataset_info::list_all_datasets_has_info(conn);
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
