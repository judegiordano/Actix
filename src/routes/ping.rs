use actix_web::{get, web, Responder};

use crate::types::structs::*;

#[get("")]
async fn get_ping() -> impl Responder {
    let response = Ping { ok: true };
    return web::Json(response);
}

pub fn configure_dev_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(get_ping));
}
