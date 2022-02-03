use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct Ping {
    pub ok: bool,
}
