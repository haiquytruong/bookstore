use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod accounts;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("NFT Bookstore project - Hai Truong")
}

pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(accounts::configure()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
