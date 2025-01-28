mod domain;

use dotenv::from_path;
use sqlx::Error;
mod config { pub mod database; }
use config::database::get_pool;

use validator::Validate;
mod dto{ pub mod login_request;pub mod register_request; }
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;

mod repository {pub mod auth_repository;}
use repository::auth_repository::AuthRepository;

use domain::user::User;
use chrono::Utc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    let auth_repository = AuthRepository::new(pool);
    let timestamp_millis = Utc::now().timestamp_millis();

    let register = RegisterRequest{
        username: "ehehehe".to_string(),
        password: "muhewhewh".to_string(),
        email: "nande@gmail.com".to_string(),
    };

    // Validasi input dulu
    match register.validate() {
        Ok(_) => {
            // mapping
            let req_register = User{
                id: 0,
                username: register.username,
                password: register.password,
                email: register.email,
                created_at: timestamp_millis,
                updated_at: timestamp_millis,
            };
            // Jika validasi sukses, lakukan registrasi
            match auth_repository.register(&req_register).await {
                Ok(true) => println!("Registrasi berhasil!"),
                Ok(false) => println!("Registrasi gagal: Username atau email mungkin sudah ada"),
                Err(e) => println!("Error saat registrasi: {:?}", e)
            }
        },
        Err(e) => println!("Validation error: {:?}", e),
    }

    Ok(())
}

#[test]
fn test_dotenv_load() {
    // Muat file .env secara eksplisit menggunakan from_path
    from_path(".env").expect("Gagal memuat file .env");

    // Ambil variabel dari lingkungan
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST tidak ditemukan");
    assert_eq!(server_host, "localhost");
}
#[tokio::test]
async fn test_connection()-> Result<(),Error>{
    let pool = get_pool().await?;

    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;


    assert!(!result.is_empty(), "Table users should not be empty");
    Ok(())
}
