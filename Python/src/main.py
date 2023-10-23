import pandas as pd
import time
import os

def compute_stats_and_export():
    # Read dataset
    start_time = time.time()
    data_path = os.path.join(os.environ['GITHUB_WORKSPACE'], "Data", "winequality-red.csv")
    data = pd.read_csv(data_path)

    # Compute statistics
    mean = data.mean()
    median = data.median()
    std = data.std()
    end_time = time.time()

    # Time taken to compute statistics
    time_taken = end_time - start_time

    # Printing results instead of writing to Excel
    print("Mean:\n", mean)
    print("\nMedian:\n", median)
    print("\nStandard Deviation:\n", std)
    print(f"\nStatistics computed in {time_taken:.2f} seconds.")

if __name__ == "__main__":
    compute_stats_and_export()
