import pandas as pd
import time

# Load the Parquet file into a DataFrame
start_time = time.time()
df = pd.read_parquet('taxi.parquet')
load_time = time.time() - start_time

# Perform a simple query
start_time = time.time()
columns_list = df.columns.tolist()
print(' '.join(columns_list))
# less than 23 miles
result = df[df['trip_distance'] < 10]
# Print the list of columns
query_time = time.time() - start_time

# Display the result
print("Query result:")
print(result.head())

# Print timing information
print(f"Loading time: {load_time} seconds")
print(f"Query time: {query_time} seconds")
