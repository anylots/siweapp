mod app;
use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let service = Router::new()
        .route("/sign_in", post(app::process_sign_in))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3030
    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(service.into_make_service())
        .await
        .unwrap();
}
