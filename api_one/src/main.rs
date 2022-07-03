#[macro_use]
extern crate diesel;
extern crate dotenv;

mod routes;
mod schemas;
mod utils;
mod models;
mod schema;

use actix_web::{web, App, HttpServer, Responder};
use serde_json::json;

use routes::month::month;
use routes::users::get_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1".to_string();
    let port = 3002;
    println!("Starting server on http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(month)
            .service(get_users)
    })
        .bind((host, port))?
        .run()
        .await
}
