mod handler;
mod model;
mod router;

use ntex::web::{self, App};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("AURA starting on http://127.0.0.1:7878");

    web::HttpServer::new(|| App::new().configure(router::routes ))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
