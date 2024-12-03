use rusqlite::{params, Connection};

use crate::data_models::dataset::Dataset;

pub fn initialize_datasets_table(conn: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS datasets (
        _id TEXT PRIMARY KEY,
        id TEXT,
        author TEXT,
        disabled BOOL,
        gated BOOL,
        last_modified DATETIME,
        likes BIGINT,
        trending_score BIGINT,
        private BOOL,
        sha TEXT,
        description TEXT,
        downloads BIGINT,
        tags TEXT,
        created_at DATETIME,
        key TEXT);
";
    conn.execute(query, []).unwrap();

    // TODO: Add indexes
}

pub fn upsert_dataset(conn: &Connection, dataset: &Dataset) {
    let query = "
    INSERT OR REPLACE INTO datasets (
        _id,
        id,
        author,
        disabled,
        gated,
        last_modified,
        likes,
        trending_score,
        private,
        sha,
        description,
        downloads,
        tags,
        created_at,
        key)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
";
    conn.execute(
        query,
        params![
            dataset._id,
            dataset.id,
            dataset.author,
            dataset.disabled,
            dataset.gated.to_string(),
            dataset.last_modified,
            dataset.likes,
            dataset.trending_score,
            dataset.private,
            dataset.sha,
            dataset.description,
            dataset.downloads,
            dataset.tags.join(";"),
            dataset.created_at,
            dataset.key
        ],
    )
    .unwrap();
}

pub fn list_datasets(conn: &Connection) -> Vec<Dataset> {
    let mut stmt = conn.prepare("SELECT * from datasets").unwrap();
    let datasets_iter = stmt.query_map([], |row| {
        let gated_str: String = row.get("gated").unwrap();
        let tags_str: String = row.get("tags").unwrap();
        let tags = tags_str.split(';').map(|str| str.trim().to_string()).collect();
        Ok(Dataset {
            _id: row.get("_id").unwrap(),
            id: row.get("id").unwrap(),
            author: row.get("author").unwrap(),
            disabled: row.get("disabled").unwrap(),
            gated: serde_json::from_str(&gated_str).unwrap(),
            last_modified: row.get("last_modified").unwrap(),
            likes: row.get("likes").unwrap(),
            trending_score: row.get("trending_score").unwrap(),
            private: row.get("private").unwrap(),
            sha: row.get("sha").unwrap(),
            description: row.get("description").unwrap(),
            downloads: row.get("downloads").unwrap(),
            tags: tags,
            created_at: row.get("created_at").unwrap(),
            key: row.get("key").unwrap(),
        }
        )
    }).unwrap();
    
    let mut datasets: Vec<Dataset> = vec![];
    for dataset in datasets_iter {
        datasets.push(dataset.unwrap())
    }
    datasets
}