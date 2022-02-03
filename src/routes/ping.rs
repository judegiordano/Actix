use actix_web::{get, web, Responder};

use crate::types::structs::*;

#[get("/")]
async fn get_ping() -> impl Responder {
    let response = Ping { ok: true };
    return web::Json(response);
}
