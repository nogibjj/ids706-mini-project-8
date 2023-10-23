# Python & Rust Performance Comparison

![GitHub Actions Status](https://github.com/aghakishiyeva/ids706-mini-project-8/workflows/main/badge.svg)

This project compares the performance of Python and Rust for calculating mean, median, and standard deviation for each column of a dataset.

## Results

- For calculating mean, median, and std of each column in **Python**, it took **0.007 seconds**. 
  [![Screenshot 2023-10-23 at 01 55 22](https://github.com/aghakishiyeva/ids706-mini-project-8/assets/78721466/a4cb6120-d33f-4942-a515-5829bd61e88d)]()

  
  To see the details, go to [x job in the successful GitHub actions link](your-github-actions-link-for-python-job).
  
- For calculating mean, median, and std of each column in **Rust**, it took **0.002 seconds**. 
  [![](link-to-your-rust-screenshot)]()
  
  To see the details, go to [x job in the successful GitHub actions link](your-github-actions-link-for-rust-job).
## Repository Structure

- `.github/workflows/main.yml`: Contains the GitHub Actions workflow that automatically runs the Python and Rust code on every push.
- `Data/winequality-red.csv`: Dataset used for the calculations.
- `Python/src/main.py`: Source code written in Python.
- `Rust/src/main.rs`: Source code written in Rust.
- `Rust/src/Cargo.toml`: Rust's package configuration file.
- `README.md`: This file, containing documentation for the repository.

## How to Run Locally

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/<your-github-username>/<your-repo-name>.git
   cd <your-repo-name>
