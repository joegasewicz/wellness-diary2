use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
}
