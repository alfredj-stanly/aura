use ntex::web;

use crate::{agent::handler::analyze, handler};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    // v0
    .route("/health", web::get().to(handler::health))
    .route("/infer", web::post().to(handler::infer))
    .route("/gaze", web::post().to(handler::gaze))
    // v1
    .route("/v1/analyze", web::post().to(analyze));
}
