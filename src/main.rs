mod db_reader;
// mod plot_utils;
use crate::db_reader::reader::SQLiteReader;
use chrono::{NaiveDateTime, Utc};
use plotters::prelude::*;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // println!("{} Hello, world!", Utc::now().timestamp());
    loop {
        let mut reader: SQLiteReader = SQLiteReader::new(PathBuf::from(
            "/mnt/nvme/git-server-bee/server_bee-backend/target/release/my_linux.db",
        ))
        .unwrap();
        let read_config_30min = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            Utc::now().timestamp() - 60 * 30,
            Utc::now().timestamp(),
            10,
        );
        let read_config_2h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            Utc::now().timestamp() - 2 * 60 * 60,
            Utc::now().timestamp(),
            60,
        );
        let read_config_6h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            Utc::now().timestamp() - 6 * 60 * 60,
            Utc::now().timestamp(),
            60,
        );
        let read_config_12h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            Utc::now().timestamp() - 12 * 60 * 60,
            Utc::now().timestamp(),
            300,
        );
        let read_config_7d = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            Utc::now().timestamp() - 7 * 24 * 60 * 60,
            Utc::now().timestamp(),
            3600,
        );

        let result1 = reader.read(read_config_30min).unwrap();
        let result2 = reader.read(read_config_2h).unwrap();
        let result3 = reader.read(read_config_6h).unwrap();
        let result4 = reader.read(read_config_12h).unwrap();
        let result5 = reader.read(read_config_7d).unwrap();

        println!("近30分钟: {:?}", result1.time_vec.len());
        println!("近2小时: {:?}", result2.time_vec.len());
        println!("近6小时: {:?}", result3.time_vec.len());
        println!("近12小时: {:?}", result4.time_vec.len());
        println!("近7天: {:?}", result5.time_vec.len());

        let data: Vec<(i64, f64)> = result3
            .time_vec
            .iter()
            .zip(result3.monitor_vec.cpu_usage.iter())
            .map(|(timestamp, cpu_usage)| (*timestamp, *cpu_usage as f64))
            .collect();
        println!("{:?}", data.len());
        sleep(Duration::from_millis(5000));
    }

    // Ok(())
}
