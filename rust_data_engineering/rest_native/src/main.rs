use axum::{http::StatusCode, routing::post, Json, Router};
use image::{Rgb, RgbImage};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::time::Instant;
use base64::encode;

#[derive(Debug, Deserialize)]
struct ComputeRequest {
    real: f64,
    imag: f64,
    width: usize,
    height: usize,
    max_iter: u32,
}

#[derive(Serialize)]
struct ComputeResponse {
    image: String,
    total_time: f64,
}

async fn compute(Json(request): Json<ComputeRequest>) -> (StatusCode, Json<ComputeResponse>) {
    let mut buffer: Vec<u32> = vec![0; request.width * request.height];
    let start_time = Instant::now();
    for y in 0..request.height {
        for x in 0..request.width {
            let cx = -2.0 + x as f64 * 3.0 / request.width as f64;
            let cy = -1.5 + y as f64 * 3.0 / request.height as f64;

            let mut zx = cx;
            let mut zy = cy;

            let c = num_complex::Complex::new(request.real, request.imag);

            let mut i = 0;
            while i < request.max_iter {
                let x_new = zx * zx - zy * zy + c.re;
                let y_new = 2.0 * zx * zy + c.im;

                if x_new * x_new + y_new * y_new > 4.0 {
                    break;
                }

                zx = x_new;
                zy = y_new;
                i += 1;
            }

            buffer[y * request.width + x] = if i == request.max_iter {
                0x000000 // Black
            } else {
                0xFFFFFF // White
            };
        }
    }

    let mut image = RgbImage::new(request.width as u32, request.height as u32);
    for y in 0..request.height {
        for x in 0..request.width {
            let pixel = buffer[y * request.width + x];
            let color = Rgb([(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8]);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    let mut cursor = Cursor::new(Vec::new());
    image
        .write_to(&mut cursor, image::ImageOutputFormat::Png)
        .unwrap();
    let image_bytes = cursor.into_inner();
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let image_base64 = encode(&image_bytes);
    let response = ComputeResponse { image: image_base64, total_time: elapsed_time };
    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/compute", post(compute));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
