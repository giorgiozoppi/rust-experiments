from fast_json import FastJsonSchema
import time
start_time = time.time()
schema = FastJsonSchema('schema.json')
json_data = schema.validate('data.json')
elapsed_time = time.time() - start_time
print(f"Elapsed time: {elapsed_time:.3f} seconds")