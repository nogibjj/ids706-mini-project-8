extern crate csv;
extern crate rayon;
extern crate rusty_machine;
extern crate chrono;
extern crate xlsxwriter;

use std::error::Error;
use std::fs::File;
use std::time::{Instant};
use rayon::prelude::*;
use rusty_machine::linalg::Vector;
use xlsxwriter::*;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    // Read the dataset
    let data_path = format!("{}/Data/winequality-red.csv", std::env::var("GITHUB_WORKSPACE").unwrap());
    let mut rdr = csv::Reader::from_path(data_path)?;
    let mut data: Vec<Vector<f64>> = vec![];

    for result in rdr.deserialize() {
        let record: Vec<f64> = result?;
        data.push(Vector::new(record));
    }

    // Compute statistics in parallel using rayon
    let means: Vector<f64> = data.par_iter().map(|v| v.mean()).collect();
    let medians: Vector<f64> = data.par_iter().map(|v| v.median()).collect();
    let std_devs: Vector<f64> = data.par_iter().map(|v| v.std_dev()).collect();

    let end_time = start_time.elapsed();
    let time_taken = end_time.as_secs() as f64 + end_time.subsec_millis() as f64 * 0.001;

    // Write to Excel
    let workbook = Workbook::new("statistics_rust.xlsx");
    let mut mean_worksheet = workbook.add_worksheet(Some("Mean")).unwrap();
    let mut median_worksheet = workbook.add_worksheet(Some("Median")).unwrap();
    let mut std_worksheet = workbook.add_worksheet(Some("Standard Deviation")).unwrap();
    let mut time_worksheet = workbook.add_worksheet(Some("Time")).unwrap();

    for (i, &val) in means.iter().enumerate() {
        mean_worksheet.write_number(i as u16, 0, val, None).unwrap();
    }

    for (i, &val) in medians.iter().enumerate() {
        median_worksheet.write_number(i as u16, 0, val, None).unwrap();
    }

    for (i, &val) in std_devs.iter().enumerate() {
        std_worksheet.write_number(i as u16, 0, val, None).unwrap();
    }

    time_worksheet.write_string(0, 0, "Time taken (sec)", None).unwrap();
    time_worksheet.write_number(0, 1, time_taken, None).unwrap();

    workbook.close().unwrap();

    println!("Statistics generated and written to Excel in {:.2} seconds.", time_taken);

    Ok(())
}
