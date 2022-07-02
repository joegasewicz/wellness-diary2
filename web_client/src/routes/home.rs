use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
pub async fn get_days(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "day": "Monday",
    });
    let body = hb.render("home", &data).unwrap();

    HttpResponse::Ok().body(body)

}