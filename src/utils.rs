use core::panic;
pub struct Config {
    pub jwt_secret: String,
    pub jwt_salt: String,
    pub jwt_expiration: String,
}

pub fn load_env() -> Config {
    dotenv::dotenv().ok();
    let JWT_SECRET = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        panic!("JWT_SECRET must be set in the environment");
    });

    let JWT_SALT = std::env::var("JWT_SALT").unwrap_or_else(|_| {
        panic!("JWT_SALT must be set in the environment");
    });
    let JWT_EXPIRATION = std::env::var("JWT_EXPIRATION").unwrap_or_else(|_| {
        panic!("JWT_EXPIRATION must be set in the environment");
    });

    return Config {
        jwt_secret: JWT_SECRET,
        jwt_salt: JWT_SALT,
        jwt_expiration: JWT_EXPIRATION,
    };
}
