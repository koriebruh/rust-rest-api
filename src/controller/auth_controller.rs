use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web::cookie::Cookie;
use chrono::Utc;
use serde_json::json;
use time::OffsetDateTime;
use crate::service::auth_service::{AuthService, IAuthService};
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::conf::jwt::{create_jwt, get_claims_from_jwt, invalidate_token};


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

    pub async fn logout(req: HttpRequest) -> impl Responder {
        //CLAIM TOKEN FROM BARRER TOKEN
        let token = req.headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth_str| auth_str.strip_prefix("Bearer "))
            .map(String::from)
            .or_else(|| req.cookie("auth_token").map(|cookie| cookie.value().to_string()));

        let token_ref = token.as_deref().unwrap_or(""); // Default ke string kosong jika None


        //MAKE COOKIE EXPERIENCED
        let expired_cookie = Cookie::build("auth_token", "")
            .path("/")
            .http_only(true)
            .secure(true)
            .max_age(time::Duration::seconds(0)) // Hapus cookie segera
            .expires(OffsetDateTime::now_utc()) // **SOLUSI: Gunakan OffsetDateTime** // Set waktu expired ke sekarang
            .finish();

        //MAKE JWT TOKEN EXPERIENCED
        if invalidate_token(token_ref).await { // <-- Gunakan `.as_str()` agar cocok dengan `&str`
            return HttpResponse::Ok()
                .cookie(expired_cookie) // Hapus cookie dari browser
                .json(json!({
                "status": "success",
                "message": "Token successfully invalidated"
            }));
        }

        // IF FAILL
        HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to invalidate token"
        }))
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
                .route("/logout", web::get().to(Self::logout))
        );
    }
}