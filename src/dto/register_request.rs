use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "username should length more than 3 digit"))]
    pub username: String,

    #[validate(length(min = 3, max = 20, message = "username should length more than 3 digit"))]
    pub password: String,

    #[validate(
        length(min = 3, max = 20, message = "username should length more than 3 digit"),
        email(message = "should contain @ ")
    )]
    pub email: String,
}