use std::env;

pub struct Config {
    pub base_url: String,
    pub ollama_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string());
        let ollama_url =
            env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());
        Config {
            base_url,
            ollama_url,
        }
    }
}
