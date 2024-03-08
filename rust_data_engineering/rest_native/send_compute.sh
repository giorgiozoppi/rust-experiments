#!/bin/bash
for i in {1..100}; do
    curl -X POST http://localhost:8001/compute -H "Content-Type: application/json" -d '{ "real": -0.8,"imag": 0.156, "width": 800,"height": 600, "max_iter": 100000}' | jq '.total_time'
done
