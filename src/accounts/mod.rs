use actix_web::{web, Responder, Scope};

async fn index() -> impl Responder {
    "Hello world!"
}

pub fn configure() -> Scope {
    web::scope("/accounts")
        .route("", web::get().to(index))
}
