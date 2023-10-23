extern crate csv;
extern crate rayon;
extern crate rusty_machine;

use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use rayon::prelude::*;
use rusty_machine::linalg::Vector;

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

    // Printing results
    println!("Mean:\n{:?}", means);
    println!("Median:\n{:?}", medians);
    println!("Standard Deviation:\n{:?}", std_devs);
    println!("Statistics computed in {:.2} seconds.", time_taken);

    Ok(())
}
