use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    id: i64,
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().route("/users", web::get().to(get_users)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
