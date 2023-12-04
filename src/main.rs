mod db_reader;
// mod plot_utils;
use crate::db_reader::reader::{ReadConfig, SQLiteReader};
use chrono::{NaiveDateTime, Utc};
use plotters::prelude::*;
use std::f32::{INFINITY, NEG_INFINITY};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

fn sleep_until(target_time: Instant) {
    let now = Instant::now();
    if now < target_time {
        let sleep_duration = target_time - now;
        // println!("{:?}", sleep_duration);
        thread::sleep(sleep_duration);
    }
}
fn main() {
    // println!("{} Hello, world!", Utc::now().timestamp());
    let mut next_execution = Instant::now();
    loop {
        let now = Instant::now();
        println!("当前时间：{:?}", SystemTime::now());
        next_execution += Duration::from_millis(2000);
        /* ---------------------------------------------------------------- */
        // println!("目标时间{:?}", next_execution);
        let mut reader: SQLiteReader =
            SQLiteReader::new(PathBuf::from("/mnt/my_linux.db")).unwrap();
        /* ---------------------------------------------------------------- */
        let now_timestamp = Utc::now().timestamp();
        let read_config_30min = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            now_timestamp - 60 * 30,
            now_timestamp,
            10,
        );
        let read_config_2h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            now_timestamp - 2 * 60 * 60,
            now_timestamp,
            60,
        );
        let read_config_6h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            now_timestamp - 6 * 60 * 60,
            now_timestamp,
            60,
        );
        let read_config_12h = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            now_timestamp - 12 * 60 * 60,
            now_timestamp,
            300,
        );
        let read_config_7d = db_reader::reader::ReadConfig::new(
            vec!["all".to_string()],
            now_timestamp - 7 * 24 * 60 * 60,
            now_timestamp,
            3600,
        );
        // println!("[{}]", now_timestamp);

        let result1 = reader.read_with_fill(&read_config_30min).unwrap();
        // let result2 = reader.read_with_fill(&read_config_2h).unwrap();
        // let result3 = reader.read_with_fill(&read_config_6h).unwrap();
        // let result4 = reader.read_with_fill(&read_config_12h).unwrap();
        // let result5 = reader.read_with_fill(&read_config_7d).unwrap();

        println!("近30分钟: {:?}", result1.monitor_vec.cpu_usage);
        // println!("近2小时: {:?}", result2.time_vec.len());
        // println!("近6小时: {:?}", result3.time_vec.len());
        // println!("近12小时: {:?}", result4.time_vec.len());
        // println!("近7天: {:?}", result5.time_vec.len());
        // println!("数据长度：{}", result3.time_vec.len());

        let plot_data = result1.clone();

        let data_cpu: Vec<(i64, f64)> = plot_data
            .time_vec
            .iter()
            .zip(plot_data.monitor_vec.cpu_usage.iter())
            .map(|(timestamp, data)| (*timestamp, *data as f64))
            .collect();
        let data_load_1: Vec<(i64, f64)> = plot_data
            .time_vec
            .iter()
            .zip(plot_data.monitor_vec.load_1.iter())
            .map(|(timestamp, data)| (*timestamp, *data as f64))
            .collect();
        let data_load_5: Vec<(i64, f64)> = plot_data
            .time_vec
            .iter()
            .zip(plot_data.monitor_vec.load_5.iter())
            .map(|(timestamp, data)| (*timestamp, *data as f64))
            .collect();
        let data_load_15: Vec<(i64, f64)> = plot_data
            .time_vec
            .iter()
            .zip(plot_data.monitor_vec.load_15.iter())
            .map(|(timestamp, data)| (*timestamp, *data as f64))
            .collect();
        let min_x = data_cpu[0].0 as i32;
        let max_x = data_cpu.last().unwrap().0 as i32;
        let data: Vec<Vec<(i64, f64)>> = vec![data_load_1, data_load_5, data_load_15, data_cpu];
        // println!("{:?}", result3.monitor_vec.cpu_usage);

        // 创建 BitMapBackend
        let root = BitMapBackend::new("plot.png", (1280, 720)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let subplots = root.split_evenly((2, 2));
        let mut series_data_iter = data.iter();
        for (i, subplot) in subplots.iter().enumerate() {
            let series_data: Vec<(i32, f64)> = series_data_iter
                .next()
                .unwrap()
                .iter()
                .map(|&(x, y)| (x as i32, y))
                .collect();
            let (min_y, max_y) = series_data
                .iter()
                .map(|(_, value)| value)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &value| {
                    (min.min(value), max.max(value))
                });
            // 创建 Chart
            let mut chart = ChartBuilder::on(subplot)
                .caption(format!("subplot {}", i), ("sans-serif", 40).into_font())
                .build_ranged(min_x..max_x, min_y..max_y)
                .unwrap();

            // 绘制坐标轴
            chart
                .configure_mesh()
                .x_desc("Timestamp")
                .y_desc("CPU Usage (%)")
                .draw()
                .unwrap();

            // println!("{:?}", series_data);
            // 绘制数据
            chart
                .draw_series(LineSeries::new(series_data, &RED))
                .unwrap();
        }
        /* ---------------------------------------------------------------- */
        // let now_timestamp = Utc::now().timestamp();
        // let read_config = db_reader::reader::ReadConfig::new(
        //     vec!["all".to_string()],
        //     now_timestamp - 7 * 24 * 60 * 60,
        //     now_timestamp,
        //     3600,
        // );
        // let reload_result_1m = reader.read_with_fill(&read_config).unwrap();
        // println!("{:?}", reload_result_1m.monitor_vec);

        /* ---------------------------------------------------------------- */
        // let mut reader = SQLiteReader::new(PathBuf::from("/mnt/my_linux.db")).unwrap();
        // let read_timestamp = Utc::now().timestamp();
        // let read_config_1min = ReadConfig::new(
        //     vec!["all".to_string()],
        //     read_timestamp - 60 * 1,
        //     read_timestamp,
        //     10,
        // );
        // let read_config_5min = ReadConfig::new(
        //     vec!["all".to_string()],
        //     read_timestamp - 60 * 5,
        //     read_timestamp,
        //     60,
        // );
        // let read_config_1hour = ReadConfig::new(
        //     vec!["all".to_string()],
        //     read_timestamp - 60 * 5 * 12,
        //     read_timestamp,
        //     300,
        // );
        // /* ----------------------- 1分钟的数据从10秒的数据读取 ----------------------- */
        // let mut load_1min_data = reader
        //     .read_with_fill(&read_config_1min)
        //     .unwrap()
        //     .monitor_vec;
        // /* ------------------------ 5分钟的数据从1分钟的数据读取 ----------------------- */
        // let mut load_5min_data = reader
        //     .read_with_fill(&read_config_5min)
        //     .unwrap()
        //     .monitor_vec;
        // /* ------------------------- 1小时的数据从5分钟读取 ------------------------- */
        // let mut load_1h_data = reader
        //     .read_with_fill(&read_config_1hour)
        //     .unwrap()
        //     .monitor_vec;
        // println!(
        //     "1min:{}; 5min:{}; 1hour:{};",
        //     load_1min_data.len, load_5min_data.len, load_1h_data.len
        // );
        /* ---------------------------------------------------------------- */
        sleep_until(next_execution);
        // println!("执行时间：{:?}", (Instant::now() - now).as_millis());
    }

    // Ok(())
}
