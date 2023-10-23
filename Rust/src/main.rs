extern crate csv;
extern crate rayon;
extern crate rusty_machine;
extern crate xlsxwriter;

use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use rayon::prelude::*;
use rusty_machine::linalg::Vector;
use xlsxwriter::*;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    // Read the dataset
    let data_path = PathBuf::from(std::env::var("GITHUB_WORKSPACE")?)
        .join("Data")
        .join("winequality-red.csv");
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
    let mut mean_worksheet = workbook.add_worksheet(Some("Mean"))?;
    let mut median_worksheet = workbook.add_worksheet(Some("Median"))?;
    let mut std_worksheet = workbook.add_worksheet(Some("Standard Deviation"))?;
    let mut time_worksheet = workbook.add_worksheet(Some("Time"))?;

    for (i, &val) in means.iter().enumerate() {
        mean_worksheet.write_number(i as u16, 0, val, None)?;
    }

    for (i, &val) in medians.iter().enumerate() {
        median_worksheet.write_number(i as u16, 0, val, None)?;
    }

    for (i, &val) in std_devs.iter().enumerate() {
        std_worksheet.write_number(i as u16, 0, val, None)?;
    }

    time_worksheet.write_string(0, 0, "Time taken (sec)", None)?;
    time_worksheet.write_number(0, 1, time_taken, None)?;

    workbook.close()?;

    println!("Statistics generated and written to Excel in {:.2} seconds.", time_taken);

    Ok(())
}
