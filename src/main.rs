use actix_web::{web, App, HttpServer};

// routes
pub mod routes;
pub use routes::ping::*;
pub use routes::user::*;
// types
pub mod types;
pub use types::structs::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started on http://127.0.0.1:8080/api");
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .configure(configure_user_scope)
                .configure(configure_dev_scope),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
