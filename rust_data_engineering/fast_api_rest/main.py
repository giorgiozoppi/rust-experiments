from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List
from numba import complex64
from numpy import complex128
import numpy as np
from datetime import datetime

MAX_ITER = 1000

app = FastAPI()

def mandelbrot(c: complex128) -> int:
    z = complex(0, 0)
    for i in range(MAX_ITER):
        z = z * z + c
        if abs(z) > 2:
            return i
    return MAX_ITER

class Point(BaseModel):
    re: float
    im: float

class Request(BaseModel):
    points: List[Point]

class Response(BaseModel):
    iterations: List[int]
    total_time: float

@app.post("/compute/")
async def compute_mandelbrot(request: Request):
    start_time = datetime.now()
    iterations = []
    for point in request.points:
        c = complex(point.re, point.im)
        iterations.append(mandelbrot(c))
    total_time = (datetime.now() - start_time).total_seconds()
    return {"iterations": iterations, "total_time": total_time}
