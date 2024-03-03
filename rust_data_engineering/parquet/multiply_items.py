import polars as pl

# Read the Parquet file into a DataFrame
df = pl.read_parquet('taxi.parquet')

# Duplicate each row 10 times
df_multiplied = df.clone().extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone()).extend(df.clone())

# Write the multiplied DataFrame to a new Parquet file
df_multiplied.write_parquet('taxi_last.parquet')