use reqwest::Client;
use rusqlite::{params, Connection};

use crate::{
    data_models::dataset_stats::DatasetStatsResponse,
    db::{self, dataset_info::DatasetInfoWrapper},
};

pub async fn fetch_and_save_all_datasets_stats(conn: &Connection) {
    db::dataset_stats::initialize_dataset_stats_table(conn);

    let datasets_info = db::dataset_info::list_all_datasets_has_info(conn);
    let params = params![];
    fetch_and_save_datasets_stats(conn, datasets_info, params).await;
}

pub async fn fetch_and_datasets_missing_stats(conn: &Connection) {
    db::dataset_stats::initialize_dataset_stats_table(conn);

    let datasets_info = db::dataset_info::list_datasets_has_info_but_no_stats(conn);
    let params = params![];
    fetch_and_save_datasets_stats(conn, datasets_info, params).await;
}

pub async fn fetch_and_save_dataset_stats(conn: &Connection, dataset_id: &str) {
    let datasets_info = db::dataset_info::get_dataset_info(conn);
    let params = params![dataset_id];
    fetch_and_save_datasets_stats(conn, datasets_info, params).await;
}

async fn fetch_and_save_datasets_stats(
    conn: &Connection,
    mut datasets_info: DatasetInfoWrapper<'_>,
    params: &[&dyn rusqlite::ToSql],
) {
    let client = Client::new();

    for dataset_info in datasets_info.get_iter(params) {
        let (id, dataset_info_response) = dataset_info.unwrap();

        for (config_name, dataset_info) in dataset_info_response.dataset_info {
            for (split_name, _) in dataset_info.splits {
                let url: String = format!(
                    "https://datasets-server.huggingface.co/statistics?dataset={}&config={}&split={}",
                    id, config_name, split_name
                );
                println!("processing dataset: {}", url);

                match client.get(&url).send().await {
                    Ok(response) => {
                        let status_code = response.status();
                        if !status_code.is_success() {
                            db::dataset_stats::upsert_dataset_stats(
                                conn,
                                &id,
                                &config_name,
                                &split_name,
                                None,
                                status_code,
                                Some(&response.text().await.unwrap()),
                            );
                        } else {
                            let dataset_stats_response =
                                response.json::<DatasetStatsResponse>().await;
                            if let Err(err) = dataset_stats_response {
                                eprintln!("Failed to deserialize with error {:?}", err);
                                return;
                            } else if let Ok(dataset_stats_response) = dataset_stats_response {
                                db::dataset_stats::upsert_dataset_stats(
                                    conn,
                                    &id,
                                    &config_name,
                                    &split_name,
                                    Some(&dataset_stats_response),
                                    status_code,
                                    None,
                                );
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to send out request: {url} with error {:?}", err);
                    }
                }
            }
        }
    }
}
