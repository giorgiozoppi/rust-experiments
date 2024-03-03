import polars as pl
import time

# Measure the time taken to load the Parquet file
start_time = time.time()

# Read the Parquet file into a DataFrame

df = pl.read_parquet('taxi.parquet')

load_time = time.time() - start_time
print(f"Loading time: {load_time} seconds")

# Perform the filter operation
start_time = time.time()

# Filter rows where the 'tripdistance' column is less than 2 miles
filtered_df = df.filter(pl.col('trip_distance') < 10)

query_time = time.time() - start_time
print(f"Query time: {query_time} seconds")
# Print the filtered DataFrame
#print("Filtered DataFrame:")
#print(filtered_df)
