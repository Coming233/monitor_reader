mod db_reader;
// mod plot_utils;
use crate::db_reader::reader::SQLiteReader;
use chrono::Utc;
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

        println!("近30分钟: {:?}", result1.cpu_usage.len());
        println!("近2小时: {:?}", result2.cpu_usage.len());
        println!("近6小时: {:?}", result3.cpu_usage.len());
        println!("近12小时: {:?}", result4.cpu_usage.len());
        println!("近7天: {:?}", result5.cpu_usage.len());

        //创建具有800x600像素区域的绘图区域
        let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        // Create a chart context
        let mut chart = ChartBuilder::on(&root)
            .caption("VecDeque Line Plot", ("sans-serif", 40).into_font())
            .build_ranged(0..180, 0.0..100.0)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Index")
            .y_desc("Y Axis")
            .draw()
            .unwrap();
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                result1
                    .cpu_usage
                    .iter()
                    .enumerate()
                    .map(|(i, y)| (i as i32, *y as f64)),
                &RED,
            ))
            .unwrap();
        sleep(Duration::from_millis(5000));
    }

    // Ok(())
}
