mod handler;
mod model;
mod openai;
mod router;
mod vision;

use ntex::web::{self, App};
use openai::OpenAIClient;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Read API key from env
    let api_key =
        std::env::var("OPEN_API_KEY").expect("OPEN_API_KEY environment variable must be set.");
    println!("api key: {}", api_key);

    let openai_client = OpenAIClient::new(api_key); 

    println!("AURA starting on http://127.0.0.1:7878");

    web::HttpServer::new(move || App::new().state(openai_client.clone()).configure(router::routes))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
