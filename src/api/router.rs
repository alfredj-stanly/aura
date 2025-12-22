use crate::api::handler;
use ntex::web::{self, HttpResponse};

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("As strong as an Ox!")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/v1/infer", web::post().to(handler::infer));
}
