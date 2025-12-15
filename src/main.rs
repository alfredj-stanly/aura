mod agent;
mod api;
mod core;
mod data;

use ntex::web::{self, App};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("AURA running on http://127.0.0.1:7878");

    web::HttpServer::new(move || App::new().configure(api::configure))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
