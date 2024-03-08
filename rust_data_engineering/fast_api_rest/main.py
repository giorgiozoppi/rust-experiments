from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List
from PIL import Image
from io import BytesIO
import time
import numpy as np
import base64 as nb

app = FastAPI()

class ComputeRequest(BaseModel):
    real: float
    imag: float
    width: int
    height: int
    max_iter: int

class ComputeResponse(BaseModel):
    image: bytes
    total_time: float

def compute_julia(request: ComputeRequest) -> ComputeResponse:
    start_time = time.time()
    img = np.zeros((request.height, request.width, 3), dtype=np.uint8)

    def julia(cx, cy, max_iter):
        zx = cx
        zy = cy
        for i in range(max_iter):
            x_new = zx * zx - zy * zy + request.real
            y_new = 2.0 * zx * zy + request.imag

            if x_new * x_new + y_new * y_new > 4.0:
                return i
            zx = x_new
            zy = y_new
        return max_iter

    for y in range(request.height):
        for x in range(request.width):
            cx = -2.0 + x * 3.0 / request.width
            cy = -1.5 + y * 3.0 / request.height
            iterations = julia(cx, cy, request.max_iter)
            if iterations == request.max_iter:
                img[y, x] = [0, 0, 0]  # Black
            else:
                img[y, x] = [255, 255, 255]  # White

    image = Image.fromarray(img)
    bytes_io = BytesIO()
    image.save(bytes_io, format='PNG')
    image_bytes = bytes_io.getvalue()
    image_base64 = nb.b64encode(image_bytes).decode('utf-8')

    elapsed_time = time.time() - start_time
    return ComputeResponse(image=image_base64, total_time=elapsed_time)

@app.post("/compute/", response_model=ComputeResponse)
async def compute(request: ComputeRequest):
    return compute_julia(request)


