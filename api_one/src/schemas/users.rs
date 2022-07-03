use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
}
