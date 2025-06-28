use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use num_bigint::BigUint;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Parse the request path to get the Fibonacci number
    let path = req.uri().path();
    let segments: Vec<&str> = path.split('/').collect();
    
    // Default to Fibonacci(10) if no number provided
    let n: u64 = segments.last()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    
    // Limit to prevent excessive computation
    let n = n.min(100);
    
    let fibonacci_result = calculate_fibonacci(n);
    
    let response_body = json!({
        "fibonacci": fibonacci_result.to_string(),
        "n": n,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "status": "success"
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(response_body.to_string().into())?)
}

fn calculate_fibonacci(n: u64) -> BigUint {
    if n == 0 {
        return BigUint::from(0u32);
    }
    if n == 1 {
        return BigUint::from(1u32);
    }
    
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);
    
    for _ in 2..=n {
        let next = &a + &b;
        a = b;
        b = next;
    }
    
    b
}