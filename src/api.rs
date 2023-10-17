use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn handle_image() -> impl Responder {
    HttpResponse::Ok().body("test")
}
