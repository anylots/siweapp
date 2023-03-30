mod app;
use axum::{routing::post, Router};
use siweapp::limiter;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

// 使用 Arc 和 Mutex 来定义全局变量 my_global_var
lazy_static::lazy_static! {
    pub static ref TOKEN_BUCKET: Arc<Mutex<limiter::TokenBucket>> =
        Arc::new(Mutex::new(limiter::TokenBucket::new(100)));
}

#[tokio::main]
async fn main() {
    let service = Router::new()
        .route("/sign_in", post(app::process_sign_in))
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(TOKEN_BUCKET.clone()))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3030
    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(service.into_make_service())
        .await
        .unwrap();
}
