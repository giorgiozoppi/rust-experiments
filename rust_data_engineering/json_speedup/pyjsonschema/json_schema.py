import json
import random
import string
import time
from jsonschema import validate

def generate_random_name(length=8):
    return ''.join(random.choices(string.ascii_letters, k=length))

def generate_random_age(min_age=18, max_age=80):
    return random.randint(min_age, max_age)

# Load schema from file
with open('schema.json', 'r') as schema_file:
    schema = json.load(schema_file)

# Load data from file
with open('data.json', 'r') as data_file:
    data = json.load(data_file)

start_time = time.time()

# Validate data against schema
try:
    for item in data:
        validate(instance=item, schema=schema)
    print("Validation passed for all items.")
except Exception as e:
    print(f"Validation failed: {e}")

elapsed_time = time.time() - start_time
print(f"Elapsed time: {elapsed_time:.3f} seconds")
