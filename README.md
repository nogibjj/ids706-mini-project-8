# Python & Rust Performance Comparison

![CI](https://github.com/aghakishiyeva/ids706-mini-project-8/actions/workflows/main.yml/badge.svg)

This project compares the performance of Python and Rust for calculating mean, median, and standard deviation for each column of a dataset.

## Results

- For calculating mean, median, and std of each column in **Python**, it took **0.008 seconds**. <br>
  [![Python Result](https://github.com/aghakishiyeva/ids706-mini-project-8/assets/78721466/a4cb6120-d33f-4942-a515-5829bd61e88d)]()

- For calculating mean, median, and std of each column in **Rust**, it took **0.002 seconds**. <br>
  [![Rust Result](https://github.com/aghakishiyeva/ids706-mini-project-8/assets/78721466/90ddd636-58b0-4c86-ae3e-851053ccf0d3)]()

For a detailed breakdown, navigate to the successful workflow in the Actions tab or directly access it through this [link](https://github.com/aghakishiyeva/ids706-mini-project-8/actions/runs/6609284941). 
  
## Repository Structure

* .github/workflows/main.yml: Contains the GitHub Actions workflow that automatically runs the Python and Rust code on every push.
* Data/winequality-red.csv: Dataset used for the calculations.
* Python/src/main.py: Source code written in Python.
* Python/requirements.txt: Lists the Python dependencies required to run the Python code.
* Rust/src/main.rs: Source code written in Rust.
* Rust/src/Cargo.toml: Rust's package configuration file, listing Rust dependencies and other metadata.
* README.md: This file, containing documentation for the repository.


## How to Run Locally

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/aghakishiyeva/ids706-mini-project-8.git
   cd ids706-mini-project-8
   ```

2. Running Python code:

   ```bash
   cd Python/src
   python main.py
   ```

3. Running Rust code:

   ```bash
   cd Rust
   cargo run
   ```

   

   
