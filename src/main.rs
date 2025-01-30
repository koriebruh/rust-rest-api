mod domain;
mod service;
mod controller;
use actix_web::{App, HttpServer, web};

mod config { pub mod database; }
use config::database::get_pool;
use validator::Validate;
mod dto {
    pub mod login_request;
    pub mod register_request;
}

mod repository { pub mod auth_repository; }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get database pool using your custom function
    let pool = match get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    let auth_repository = repository::auth_repository::AuthRepository::new(pool.clone());
    let auth_service = service::auth_service::AuthService::new(auth_repository);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .configure(|cfg| {
                controller::auth_controller::AuthController::configure_routes(cfg)
            })
    })
        .bind("localhost:3000")?
        .run()
        .await
}

