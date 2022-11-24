use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pong))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
