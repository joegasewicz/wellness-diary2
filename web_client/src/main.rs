use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
async fn get_days(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "day": "Monday",
    });
    let body = hb.render("home", &data).unwrap();

    HttpResponse::Ok().body(body)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1".to_string();
    let port = 3000;
    println!("Starting server on http://{}:{}", host, port);

    // Templating
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("hbs", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(get_days)
    })
        .bind((host, port))?
        .run()
        .await
}
