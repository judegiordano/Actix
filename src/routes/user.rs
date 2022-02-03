use actix_web::{get, web, Responder};

use crate::types::structs::*;

#[get("")]
async fn get_users() -> impl Responder {
    let mut users: Vec<User> = Vec::new();
    users.push(User {
        username: "user_1".to_string(),
        id: 897364597836,
    });
    users.push(User {
        username: "user_2".to_string(),
        id: 2837402847345,
    });
    users.push(User {
        username: "user_3".to_string(),
        id: 2349823689274,
    });
    return web::Json(users);
}

pub fn configure_user_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(get_users));
}
