use dotenv::from_path;
use sqlx::Error;
mod config { pub mod database; }
use config::database::get_pool;

fn main() {
    println!("dsds")
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
