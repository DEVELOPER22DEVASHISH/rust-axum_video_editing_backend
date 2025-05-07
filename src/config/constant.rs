use std::env;

pub struct EnvConfig {
    // pub upload_dir: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn init() -> Self {
        dotenvy::dotenv().ok(); // Load from .env automatically

        // let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());

        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(5000);

        // Self { upload_dir, port }
        Self {  port }
    }
}
