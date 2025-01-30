use actix_web::{web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use serde_json::json;

use crate::service::auth_service::{AuthService, IAuthService};
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::conf::jwt::create_jwt;



pub struct AuthController {
    auth_service: AuthService,
}

impl AuthController {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }

    pub async fn login(
        auth_service: web::Data<AuthService>,
        request: web::Json<LoginRequest>,
    ) -> impl Responder {
        // Extract username before consuming the request
        let username = request.username.clone();

        match auth_service.login(request.into_inner()).await {
            Ok(_) => {
                match create_jwt(&username).await {
                    Ok(token) => {
                        let cookie = Cookie::build("auth_token", token.clone())
                            .path("/")
                            .http_only(true)
                            .secure(true)
                            .max_age(time::Duration::hours(24))
                            .finish();

                        HttpResponse::Ok()
                            .cookie(cookie)
                            .json(json!({
                                "status": "success",
                                "message": "Login successful",
                                "token": token
                            }))
                    }
                    Err(err) => HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": err.to_string()
                    })),
                }
            }
            Err(err) => HttpResponse::Unauthorized().json(json!({
                "status": "error",
                "message": err.to_string()
            })),
        }
    }


    pub async fn register(
        auth_service: web::Data<AuthService>,
        request: web::Json<RegisterRequest>,
    ) -> impl Responder {
        match auth_service.register(request.into_inner()).await {
            Ok(message) => HttpResponse::Created().json(json!({
                "status": "success",
                "message": message
            })),
            Err(err) => HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": err.to_string()
            }))
        }
    }

    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().json(json!({
            "message": "Hello, world!"
        }))
    }
    // Optional: Method to configure routes
    pub fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auth")
                .route("/hello", web::get().to(Self::hello))
                .route("/login", web::post().to(Self::login))
                .route("/register", web::post().to(Self::register))
        );
    }
}