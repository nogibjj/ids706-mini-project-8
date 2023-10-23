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

    // Compute means in parallel using rayon
    let means_vec: Vec<f64> = data.par_iter().map(|v| v.mean()).collect();
    let means = Vector::new(means_vec);

    // Compute medians (placeholder for now)
    // TODO: Implement the median computation or use another crate
    let medians = means.clone(); 

    // Compute standard deviations (placeholder for now)
    // TODO: Implement the standard deviation computation or use another crate
    let std_devs = means.clone();

    // Print statistics
    println!("Mean values:");
    for (i, mean) in means.iter().enumerate() {
        println!("Column {}: {}", i + 1, mean);
    }
    println!("\nMedian values:");
    for (i, median) in medians.iter().enumerate() {
        println!("Column {}: {}", i + 1, median);
    }
    println!("\nStandard Deviation values:");
    for (i, std_dev) in std_devs.iter().enumerate() {
        println!("Column {}: {}", i + 1, std_dev);
    }

    let end_time = start_time.elapsed();
    let time_taken = end_time.as_secs() as f64 + end_time.subsec_millis() as f64 * 0.001;

    println!("Statistics generated in {} seconds.", time_taken);

    Ok(())
}
