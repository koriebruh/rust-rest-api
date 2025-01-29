mod domain;
mod service;
mod controller;
use crate::controller::auth_controller::register;
use crate::controller::auth_controller::login;
use actix_web::{App, HttpServer, web};
mod config { pub mod database; }
use config::database::get_pool;
use validator::Validate;
mod dto{ pub mod login_request;pub mod register_request; }

mod repository {pub mod auth_repository;}
use repository::auth_repository::AuthRepository;
use crate::service::auth_service::AuthService;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await.expect("Failed to connect to database");
    let auth_repository = AuthRepository::new(pool);
    let auth_service = AuthService::new(auth_repository);

    println!("Server running at http://localhost:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .service(login)
            .service(register)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
