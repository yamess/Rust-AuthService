use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
responses(
(status = 200, description = "Healthy")
)
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}
