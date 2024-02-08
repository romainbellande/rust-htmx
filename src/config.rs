use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Auth0 {
    pub client_id: String,
    pub client_secret: String,
    pub issuer: String,
}

pub struct Config {
    pub base_url: String,
    pub database_url: String,
    pub auth0: Auth0,
}

impl Config {
    pub fn new() -> Self {
        dotenv().expect(".env file not found");

        Self {
            base_url: env::var("BASE_URL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            auth0: Auth0 {
                client_id: env::var("AUTH0_CLIENT_ID").unwrap(),
                client_secret: env::var("AUTH0_CLIENT_SECRET").unwrap(),
                issuer: env::var("AUTH0_ISSUER").unwrap(),
            },
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
