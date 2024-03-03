use axum::{http::StatusCode, routing::post, Json, Router};
use num_complex::Complex;
use std::time::Instant;
const MAX_ITER: usize = 1000;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
struct Point {
    re: f64,
    im: f64,
}
#[derive(Debug, Deserialize)]
struct MandlerbrotRequest {
    points: Vec<Point>,
}

#[derive(Debug, Serialize)]
struct MandlerbrotResponse {
    iterations: Vec<usize>,
    total_time: f64,
}
fn julia_mandelbrot(c: Complex<f64>, z: Complex<f64>) -> usize {
    let mut z = z;
    for i in 0..MAX_ITER {
        z = z * z + c;
        if z.norm() > 2.0 {
            return i;
        }
    }
    MAX_ITER
}

async fn compute_mandelbrot(
    // this argument tells axum to parse the request body
    // as JSON into a `Request` type
    Json(req): Json<MandlerbrotRequest>,
) -> (StatusCode, Json<MandlerbrotResponse>) {
    // insert your application logic here
    let start_time = Instant::now();
    let mut result = Vec::new();
    for point in req.points.iter() {
        let c = Complex::new(point.re, point.im);
        let iterations = julia_mandelbrot(c, c);
        result.push(iterations);
    }
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let response = MandlerbrotResponse {
        iterations: result,
        total_time: elapsed_time,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(response))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/compute", post(compute_mandelbrot));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
