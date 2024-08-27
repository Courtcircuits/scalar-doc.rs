use axum::{routing::get, Router};
use scalar_doc::Documentation;

async fn doc() -> String {
    Documentation::new("Api Documentation title", "/openapi")
        .build()
        .unwrap()
}

async fn openapi() -> &'static str {
    include_str!("openapi.json")
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new()
        .route("/", get(doc))
        .route("/openapi", get(openapi));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
