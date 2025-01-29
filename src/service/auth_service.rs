use chrono::Utc;
use crate::repository::auth_repository::{AuthRepository, ErrCustom};
use crate::domain::user::User;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use validator::Validate;

#[derive(Clone)]
pub(crate) struct AuthService {
    repository: AuthRepository,
}

impl AuthService {
    pub fn new(repository: AuthRepository) -> Self {
        Self { repository }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String, ErrCustom> {
        if let Err(_validation_errors) = request.validate() {
            return Err(ErrCustom::InvalidCredentials);
        }

        match self.repository.login(&request.username, &request.password).await {
            Ok(true) => Ok("success login".to_string()),
            Ok(false) => Err(ErrCustom::InvalidCredentials),
            Err(e) => Err(e),
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<String, ErrCustom> {

        if let Err(_validation_errors) = request.validate() {
            return Err(ErrCustom::InvalidCredentials);
        }

        let mapping = User {
            id: 0,
            username: request.username,
            password: request.password,
            email: request.email,
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };

        match self.repository.register(&mapping).await {
            Ok(_) => Ok("register user success ".to_string()),
            Err(e) => Err(e),
        }

    }
}