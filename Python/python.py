import pandas as pd
import time

def compute_stats_and_export():
    # Read dataset
    start_time = time.time()
    data = pd.read_csv("../Data/winequality-red.csv")

    # Compute statistics
    mean = data.mean()
    median = data.median()
    std = data.std()
    end_time = time.time()

    # Time taken to compute statistics
    time_taken = end_time - start_time

    # Writing to Excel
    with pd.ExcelWriter("statistics_python.xlsx") as writer:
        mean.to_excel(writer, sheet_name="Mean")
        median.to_excel(writer, sheet_name="Median")
        std.to_excel(writer, sheet_name="Standard Deviation")
        pd.Series({"Time taken (sec)": time_taken}).to_excel(writer, sheet_name="Time")

    print(f"Statistics generated and written to Excel in {time_taken:.2f} seconds.")

if __name__ == "__main__":
    compute_stats_and_export()
