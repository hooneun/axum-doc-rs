use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
