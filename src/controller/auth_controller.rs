use actix_web::{web, HttpResponse, Responder, post};
use crate::service::auth_service::AuthService;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::repository::auth_repository::ErrCustom;

#[derive(Clone)]
pub struct AuthController {
    service: AuthService,
}

impl AuthController {
    pub fn new(service: AuthService) -> Self {
        Self { service }
    }
}

#[post("/api/login")]
async fn login(
    service: web::Data<AuthService>,
    request: web::Json<LoginRequest>,
) -> impl Responder {
    match service.login(request.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => match e {
            ErrCustom::InvalidCredentials => HttpResponse::BadRequest().json("Invalid credentials"),
            _ => HttpResponse::InternalServerError().json("Internal server error"),
        },
    }
}

#[post("/api/register")]
async fn register(
    service: web::Data<AuthService>,
    request: web::Json<RegisterRequest>,
) -> impl Responder {
    match service.register(request.into_inner()).await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => match e {
            ErrCustom::InvalidCredentials => HttpResponse::BadRequest().json("Invalid request"),
            _ => HttpResponse::InternalServerError().json("Internal server error"),
        },
    }
}