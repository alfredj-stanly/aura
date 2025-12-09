use ntex::web::{self, App, HttpResponse};

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("As strong as an Oak!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("AURA starting on http://127.0.0.1:7878");

    web::HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
