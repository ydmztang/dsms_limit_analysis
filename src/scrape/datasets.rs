use reqwest::Client;
use rusqlite::Connection;

use crate::{data_models::dataset::Dataset, db};

pub async fn fetch_and_save_all_datasets(conn: &Connection) {
    db::datasets::initialize_datasets_table(conn);

    let client = Client::new();

    let mut url: String = "https://huggingface.co/api/datasets".to_string();
    let mut total_datasets = 0;

    loop {
        println!("Querying url: {}", &url);
        match client.get(&url).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!("Response failed for request: {url}. Retrying...");
                } else {
                    //// Debug which item can't be deserialized
                    // let response: serde_json::Value = response.json().await.unwrap();
                    // for item in response.as_array().unwrap() {
                    //     let dataset= serde_json::from_value::<Dataset>(item.clone());
                    //     if let Err(err) = dataset {
                    //         println!("failed item: {:?}", item);
                    //         println!("Error: {:?}", err);
                    //         break;
                    //     }
                    // }
                    
                    let header = response.headers().get("link");
                    let mut last_page = false;
                    match header {
                        Some(header) => {
                            // Fetch the next link from link header: https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api?apiVersion=2022-11-28#link-header
                            let header_str = header.to_str().unwrap();
                            let parts: Vec<&str> = header_str.split(">; ").collect();
                            assert!(parts.len() == 2, "Expected 2 parts of the link header. Got {}", header_str);
                            
                            url = parts[0].trim_start_matches('<').to_owned();
                            if parts[1] == "rel=\"last\"" {
                                last_page = true;
                            }
                        }
                        None => {
                            last_page = true;
                        }
                    }
      
                    let datasets = response.json::<Vec<Dataset>>().await.unwrap();
                    total_datasets += datasets.len();
                    println!("total datasets visited: {total_datasets}");

                    for dataset in datasets {
                        db::datasets::upsert_dataset(conn, &dataset);
                    }

                    if last_page {
                        println!("All pages visited!");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to send out request: {url} with error {e}. Retrying...");
            }
        }
    }   
}
