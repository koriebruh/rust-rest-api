mod domain;

use actix_web::{App, HttpServer, web};
use crate::conf::database::get_pool;

mod conf { pub mod database;  pub mod jwt;}
mod dto {
    pub mod login_request;
    pub mod register_request;
}


mod repository { pub mod auth_repository; }
mod service {pub mod auth_service;}
mod controller {pub mod auth_controller;}


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

    println!("server listen on :3000");
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

