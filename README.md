# Scalar Doc (for Rust)

An HTTP API documentation generator for Rust that doesn't care about which HTTP framework yo use.

## Usage

To use this crate, add it to your `Cargo.toml` with :
```bash
cargo add scalar_doc
```

Also, you will need **to expose your OpenAPI schema as an HTTP endpoint** since it's too complicated to find the path of the schema file (it's an open issue though so I would be really happy if you contribute it).

### Axum

```rust
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
```

### Actix

To use `scalar-doc` with `actix`, you will need to activate the `actix` feature in your `Cargo.toml` file.

```bash
cargo add scalar-doc -F actix
```

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use scalar_doc::scalar_actix::ActixDocumentation;

#[get("/")]
async fn doc() -> impl Responder {
    ActixDocumentation::new("Api Documentation title", "/openapi")
        .theme(scalar_doc::Theme::Kepler)
        .service()
}

#[get("/openapi")]
async fn openapi() -> impl Responder {
    let open = include_str!("openapi.json");
    HttpResponse::Ok().body(open)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(doc).service(openapi))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```


Check out the [examples](./examples) for more show cases.
