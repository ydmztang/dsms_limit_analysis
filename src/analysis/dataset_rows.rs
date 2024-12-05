use rusqlite::Connection;

use crate::db::dataset_info::DatasetInfoWrapper;

// Report the number every GRANULARITY datasets. This gives us roughly a data point every 1% of data
const GRANULARITY: i32 = 2000;

pub fn get_dataset_row_limit_coverage(conn: &Connection, order_by: &str, limit: i64) {
    let mut total_count = 0;
    let mut cover_count = 0;
    for dataset_info in get_ordered_dataset_info(conn, order_by).get_iter() {
        total_count += 1;

        let (_, dataset_info_response) = dataset_info.unwrap();
        let mut dataset_rows = 0;
        for (_, dataset_info) in dataset_info_response.dataset_info {
            for (_, split_info) in dataset_info.splits {
                dataset_rows += split_info.num_examples;
            }
        }

        if limit > dataset_rows {
            cover_count += 1;
        }

        if total_count % GRANULARITY == 0 {
            // TODO: Report data points
            println!("coverage: {}", cover_count as f64 / total_count as f64 * 100_f64);
        }
    }
}

fn get_ordered_dataset_info<'a>(
    conn: &'a Connection,
    order_by: &str,
) -> DatasetInfoWrapper<'a> {
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
            order_by
        ))
        .unwrap();
    DatasetInfoWrapper::new(stmt, vec![])
}
