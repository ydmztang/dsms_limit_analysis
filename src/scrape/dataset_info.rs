use reqwest::Client;
use rusqlite::Connection;

use crate::{data_models, data_models::dataset_info::DatasetInfoResponse, db};

pub async fn fetch_and_save_all_datasets_info(conn: &Connection) {
    db::dataset_info::initialize_dataset_info_table(conn);

    println!("listing all datasets");
    let datasets = db::datasets::list_all_datasets(conn);
    println!("start pulling dataset info...");

    fetch_and_save_datasets_info(conn, datasets).await;
}

pub async fn fetch_and_save_datasets_missing_info(conn: &Connection) {
    db::dataset_info::initialize_dataset_info_table(conn);

    println!("listing datasets doesn't have dataset info pulled...");
    let datasets = db::datasets::list_datasets_missing_info(conn);
    println!("start pulling dataset info...");

    fetch_and_save_datasets_info(conn, datasets).await;
}

async fn fetch_and_save_datasets_info(
    conn: &Connection,
    datasets: Vec<data_models::dataset::Dataset>,
) {
    let client = Client::new();

    for dataset in datasets {
        let url: String = format!(
            "https://datasets-server.huggingface.co/info?dataset={}",
            dataset.id
        );
        println!("processing dataset: {}", dataset.id);

        match client.get(&url).send().await {
            Ok(response) => {
                let status_code = response.status();
                if !status_code.is_success() {
                    db::dataset_info::upsert_dataset_info(
                        conn,
                        &dataset._id,
                        &dataset.id,
                        None,
                        status_code,
                        Some(&response.text().await.unwrap()),
                    );
                } else {
                    let dataset_info_response = response.json::<DatasetInfoResponse>().await;
                    if let Err(err) = dataset_info_response {
                        db::dataset_info::upsert_dataset_info(
                            conn,
                            &dataset._id,
                            &dataset.id,
                            None,
                            reqwest::StatusCode::from_u16(400).unwrap(),
                            Some(&format!("{:?}", err)),
                        );
                    } else if let Ok(dataset_info) = dataset_info_response {
                        db::dataset_info::upsert_dataset_info(
                            conn,
                            &dataset._id,
                            &dataset.id,
                            Some(&dataset_info),
                            status_code,
                            None,
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to send out request: {url} with error {e}");
            }
        }
    }
}
