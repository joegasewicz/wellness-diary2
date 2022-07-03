use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;


#[get("/diary")]
pub async fn diary(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({

    });
    let body = hb.render("diary", &data).unwrap();

    HttpResponse::Ok().body(body)
}

