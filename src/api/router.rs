use crate::api::{fuzzy, infer};
use ntex::web::{self, HttpResponse, post};

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("As strong as an Oak!")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/v1/infer", post().to(infer::handler))
        .route("/v1/fuzzy", post().to(fuzzy::handler));
}
