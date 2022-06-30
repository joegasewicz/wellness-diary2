use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::json;

#[get("/days")]
async fn get_days() -> impl Responder {
    format!("Working... ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1".to_string();
    let port = 3000;
    println!("Starting server on http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new().service(get_days)
    })
    .bind((host, port))?
    .run()
    .await
}
