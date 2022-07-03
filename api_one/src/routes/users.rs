extern crate diesel;

use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use crate::models::users::User;
use self::diesel::prelude::*;

use crate::schemas;
use crate::utils::database::establish_connection;

use crate::schema;

#[get("/users")]
pub async fn get_users() -> Result<impl Responder> {
    let conn = establish_connection();
    let result = schema::users::table
        .filter(schema::users.email.eq("test1@email.com"))
        .limit(5)
        .load::<User>(&conn)
        .expect("Error fetching users");

    let obj = schemas::users::User {
        id: 0,
        email: "email test".to_string(),
        password: "password test".to_string(),
        name: "name test".to_string(),
    };
    Ok(web::Json(obj))
}
