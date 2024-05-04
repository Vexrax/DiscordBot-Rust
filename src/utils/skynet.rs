use std::env;
use crate::utils::skynet_constants::Environment;

pub fn get_env() -> Environment {
    let env = env::var("ENV").expect("Expected ENV variable in environment");
    return match env.as_str() {
        "PROD" => Environment::PROD,
        "DEV" => Environment::DEV,
        _ => {
            panic!("Env var for ENV was not set!")
        }
    }
}