use std::collections::HashMap;

use rusqlite::Connection;

use crate::db::{self, common::OrderByOptions};

pub fn get_task_popularity(conn: &Connection, order_by: OrderByOptions, top: f64) {
    let dataset_count = db::total_dataset_count::get_dataset_count(conn);
    let target_count = (dataset_count.datasets as f64 * top) as usize;
    let (_, task_category_mapping) = get_category_task_mapping();
    println!("mapping: {:?}", task_category_mapping);

    let mut task_counts = HashMap::<String, usize>::new();
    let mut category_counts = HashMap::<String, usize>::new();
    let mut visited_count = 0;
    for dataset in db::datasets::get_ordered_datasets(conn, order_by) {
        for tag in dataset.tags {
            let tag = tag.to_lowercase();
            if tag.starts_with("task_categories:") {
                let ret: Vec<&str> = tag.split(':').collect();
                if ret.len() == 2 {
                    let task = ret[1];
                    let category = task_category_mapping.get(task).unwrap();
                    *task_counts.entry(task.to_string()).or_insert(0) += 1;
                    *category_counts.entry(category.to_string()).or_insert(0) += 1;
                }
            }
        }

        visited_count += 1;
        if visited_count >= target_count {
            break;
        }
    }

    println!("task,category,counts");
    for (task, count) in task_counts {
        println!(
            "{},{},{}",
            task,
            task_category_mapping.get(&task).unwrap(),
            count
        );
    }

    println!("------------------------------------------------------------");
    println!("category,counts");
    for (category, count) in category_counts {
        println!("{},{}", category, count);
    }
}

fn get_category_task_mapping() -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let mut category_task_mapping = HashMap::<&str, Vec<&str>>::new();
    category_task_mapping.insert(
        "multimodal",
        vec![
            "visual-question-answering",
            "video-text-to-text",
            "document-question-answering",
            "image-text-to-text",
        ],
    );
    category_task_mapping.insert(
        "computer-vision",
        vec![
            "depth-estimation",
            "image-classification",
            "object-detection",
            "image-segmentation",
            "text-to-image",
            "image-to-text",
            "image-to-image",
            "image-to-video",
            "unconditional-image-generation",
            "video-classification",
            "text-to-video",
            "zero-shot-image-classification",
            "mask-generation",
            "zero-shot-object-detection",
            "text-to-3d",
            "image-to-3d",
            "image-feature-extraction",
            "keypoint-detection",
        ],
    );
    category_task_mapping.insert(
        "natural-language-processing",
        vec![
            "text-classification",
            "token-classification",
            "table-question-answering",
            "question-answering",
            "zero-shot-classification",
            "translation",
            "summarization",
            "feature-extraction",
            "text-generation",
            "text2text-generation",
            "fill-mask",
            "sentence-similarity",
            "table-to-text",
            "multiple-choice",
            "text-retrieval",
        ],
    );
    category_task_mapping.insert(
        "audio",
        vec![
            "text-to-speech",
            "text-to-audio",
            "automatic-speech-recognition",
            "audio-to-audio",
            "audio-classification",
            "voice-activity-detection",
        ],
    );
    category_task_mapping.insert(
        "tabular",
        vec![
            "tabular-classification",
            "tabular-regression",
            "tabular-to-text",
            "time-series-forecasting",
        ],
    );
    category_task_mapping.insert(
        "reinforcement-learning",
        vec!["reinforcement-learning", "robotics"],
    );
    category_task_mapping.insert("other", vec!["graph-ml", "other"]);

    let mut category_task_mapping_str = HashMap::<String, Vec<String>>::new();
    let mut task_category_mapping = HashMap::<String, String>::new();
    for (category, tasks) in category_task_mapping {
        let mut tasks_str: Vec<String> = vec![];
        for task in tasks {
            tasks_str.push(String::from(task));
            task_category_mapping.insert(String::from(task), category.to_string());
        }
        category_task_mapping_str.insert(category.to_string(), vec![]);
    }

    (category_task_mapping_str, task_category_mapping)
}
