use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    q: Option<String>,
}

#[get("/")]
pub async fn handle_image(query: web::Query<QueryParams>) -> impl Responder {
    let q = match query.q {
        Some(ref q) => q,
        None => "World",
    };
    HttpResponse::Ok().body(format!("Hello {}!", q))
}
