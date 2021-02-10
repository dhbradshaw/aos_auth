use dotenv;
use std::env;

/// Config has exclusive responsibility for all config info including
/// 1. environment variables
/// 2. commandline arguments
#[derive(Clone)]
pub struct Config {
    pub base_url: String,
    hash_key: String,
    host_ip: String,
    port: String,
}

impl Config {
    pub fn from_envs() -> Self {
        dotenv::from_filename("envs/local.env").ok();
        env_logger::init();
        Self {
            base_url: env::var("BASE_URL").expect("BASE_URL must be set."),
            hash_key: env::var("HASH_KEY").expect(
                "HASH_KEY must be set.  
                Use generate_secret_key to get one then save it to .env .",
            ),
            host_ip: env::var("HOST_IP").expect("HOST_IP must be set"),
            port: env::var("PORT").expect("PORT must be set"),
        }
    }
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host_ip, self.port)
    }
    pub fn get_password_hasher(&self) -> auth::hashing::PasswordHasher {
        auth::hashing::PasswordHasher::new(&self.hash_key)
    }
    pub fn get_datalayer(&self) -> datalayer::DataLayer {
        let db_name = env::var("DATABASE_NAME").expect("Must supply database name.");
        datalayer::DataLayer::new(&format!("dbs/sled_{}", db_name))
    }
}

pub mod atomics;
pub mod auth;
pub mod app;
pub mod datalayer;
