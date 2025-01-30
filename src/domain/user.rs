
pub struct User {
    pub(crate) id: Option<i64>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}