mod routes;

use actix_web::{web, App, HttpServer};
use actix_files as fs;
use handlebars::Handlebars;

use routes::{home::{get_days}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1".to_string();
    let port = 3000;
    println!("Starting server on http://{}:{}", host, port);

    // Templating
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory("hbs", "./templates").unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/js", "static/js").show_files_listing())
            .app_data(handlebars_ref.clone())
            .service(get_days)
    })
        .bind((host, port))?
        .run()
        .await
}
