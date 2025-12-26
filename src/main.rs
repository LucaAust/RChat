mod db;
mod frontend;
mod ws_handle;

use crate::{frontend::routes::hello, ws_handle::ws_upgrade_handler};
use axum::{
    Router,
    routing::{any, get},
};
use db::initialize_database;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pool = initialize_database().await;

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/ws", any(ws_upgrade_handler))
        .route("/", get(hello))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
