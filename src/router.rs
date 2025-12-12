use ntex::web;

use crate::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handler::health));
    cfg.route("/infer", web::post().to(handler::infer));
    cfg.route("/gaze", web::post().to(handler::gaze));
}
