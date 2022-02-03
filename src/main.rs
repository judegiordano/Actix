use actix_web::{get, web, App, HttpServer, Responder};

mod types;

#[get("/")]
async fn get_ping() -> impl Responder {
    let response = types::Ping { ok: true };
    return web::Json(response);
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let mut users: Vec<types::User> = Vec::new();
    users.push(types::User {
        username: "user_1".to_string(),
        id: 897364597836,
    });
    users.push(types::User {
        username: "user_2".to_string(),
        id: 2837402847345,
    });
    users.push(types::User {
        username: "user_3".to_string(),
        id: 2349823689274,
    });
    return web::Json(users);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started on http://127.0.0.1:8080");
    HttpServer::new(move || App::new().service(get_users).service(get_ping))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
