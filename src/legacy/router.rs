use ntex::web;

use crate::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // v0
        .route("/health", web::get().to(handler::health))
        .route("/infer", web::post().to(handler::infer))
        .route("/gaze", web::post().to(handler::gaze));
}
