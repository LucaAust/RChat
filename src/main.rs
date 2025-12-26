mod db;
use axum::{Json, Router, routing::get};
use db::initialize_database;
use serde_json::json;

#[tokio::main]
async fn main() {
    let pool = initialize_database().await;

    let api = Router::new().route("/posts", get(|| async { Json(json!({ "data": 42 })) }));

    let app = Router::new()
        .nest("/api", api)
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
