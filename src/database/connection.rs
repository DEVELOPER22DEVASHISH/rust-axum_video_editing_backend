use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn establish_connection() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("Connecting to: {}", database_url);

    let mut opt = ConnectOptions::new(database_url.clone());
    opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(true);

    match Database::connect(opt).await {
        Ok(conn) => {
            println!("✅ Successfully connected to the database!");
            conn
        },
        Err(e) => {
            eprintln!("❌ Database connection error: {:?}", e);
            panic!("Failed to connect to the database");
        }
    }
}
