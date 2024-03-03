#!/bin/bash
curl -X POST "http://127.0.0.1:8000/compute/" -H "Content-Type: application/json" -d '{
  "points": [
    {"re": -1.0, "im": 0.0},
    {"re": 0.0, "im": 0.0},
    {"re": 0.5, "im": 0.5}
  ]
}'