use actix_web::*;

pub async fn info() -> HttpResponse {
    HttpResponse::Ok().body("conectado...")
}