use crate::db_reader::model::MonitorVec;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct ReadData {
    time_vec: Vec<i64>,
    monitor_vec: MonitorVec,
}
pub struct SQLiteReader {
    conn: Connection,
}
pub struct ReadConfig {
    metric_name: Vec<String>,
    start_time: i64,
    stop_time: i64,
    period: i64,
}
impl ReadConfig {
    pub fn new(metric_name: Vec<String>, start_time: i64, stop_time: i64, period: i64) -> Self {
        Self {
            metric_name,
            start_time,
            stop_time,
            period,
        }
    }
}
// enum TableNames {
//     ten:"DataPer10Second",
// }
impl SQLiteReader {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(db_path)?,
            // read_data: MonitorVec::new(0),
        })
    }
    pub fn read(
        &mut self,
        read_config: ReadConfig,
    ) -> Result<MonitorVec, Box<dyn std::error::Error>> {
        let table_name: String = match read_config.period {
            10 => {
                // println!("10秒");
                "DataPer10Second".to_string()
            }
            60 => {
                // println!("60秒");
                "DataPer1Minute".to_string()
            }
            300 => {
                // println!("300秒");
                "DataPer5Minute".to_string()
            }
            3600 => {
                // println!("3600秒");
                "DataPer1Hour".to_string()
            }
            86400 => {
                // println!("86400秒");
                "DataPer1Hour".to_string()
            }
            _ => {
                // println!("其他数字，默认返回10");
                "DataPer10Second".to_string()
            }
        };
        let sql: String = format!(
            "SELECT * FROM {} WHERE timestamp BETWEEN {} AND {};",
            table_name,
            read_config.start_time / read_config.period * read_config.period,
            read_config.stop_time
        );
        println!("SQL 语句是：{}", sql);

        let mut stmt: rusqlite::Statement<'_> = self.conn.prepare(&sql)?;
        let mut result_set = stmt.query([])?;

        let mut result_vec = MonitorVec::new(0);
        // 使用循环迭代结果集中的每一行
        while let Some(row) = result_set.next()? {
            result_vec.len += 1;
            result_vec.load_1.push_back(row.get(1)?);
            result_vec.load_5.push_back(row.get(2)?);
            result_vec.load_15.push_back(row.get(3)?);
            result_vec.cpu_usage.push_back(row.get(4)?);
            result_vec.memory_used.push_back(row.get(5)?);
            result_vec.memory_free.push_back(row.get(6)?);
            result_vec.swap_used.push_back(row.get(7)?);
            result_vec.swap_free.push_back(row.get(8)?);
            result_vec.disk_used.push_back(row.get(9)?);
            result_vec.disk_read.push_back(row.get(10)?);
            result_vec.disk_write.push_back(row.get(11)?);
            result_vec.network_rx.push_back(row.get(12)?);
            result_vec.network_tx.push_back(row.get(13)?);
        }
        Ok(result_vec)
    }
}
