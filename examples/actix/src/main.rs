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
