mod agent;
mod api;
mod core;
mod data;

use ntex::web::{self, App};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPEN_API_KEY environment variable must be set.\n `export OPENAI_API_KEY='...'`");

    println!("AURA running on http://127.0.0.1:7878");

    web::HttpServer::new(move || App::new().state(api_key.clone()).configure(api::configure))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
