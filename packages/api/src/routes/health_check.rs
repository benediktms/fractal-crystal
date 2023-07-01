use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    tracing::info!("health_check handler called");

    HttpResponse::Ok().finish()
}
