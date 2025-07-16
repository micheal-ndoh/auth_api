use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_salt: [u8; 16],
    pub jwt_secret: String,
    pub jwt_expiration_secs: u32,
    pub google_client_id: String,
    pub google_client_secret: String,
}

pub fn load_env() -> Config {
    dotenv().ok();

    let jwt_salt = std::env::var("JWT_SALT").expect("JWT_SALT environment variable is not set");

    let jwt_salt_bytes = jwt_salt.as_bytes();
    if jwt_salt.len() != 16 {
        panic!("JWT_SALT must be 16 bytes long");
    }
    let mut jwt_salt = [0u8; 16];
    jwt_salt.copy_from_slice(&jwt_salt_bytes[0..16]);

    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");

    let jwt_expiration = std::env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION environment variable is not set")
        .parse::<u32>()
        .expect("JWT_EXPIRATION mustv be a valid unsigned integer");

    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID environment variable is not set");
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET environment variable is not set");

    return Config {
        jwt_salt,
        jwt_secret,
        jwt_expiration_secs: jwt_expiration,
        google_client_id,
        google_client_secret,
    };
}
