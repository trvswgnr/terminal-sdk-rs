use openapi::apis;
pub use openapi::apis::configuration::Configuration as Config;
pub use openapi::models;

/// Client for the Terminal API
pub struct Client {
    pub config: Config,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

// generated client from build.rs
include!(concat!(env!("OUT_DIR"), "/api_methods_gen.rs"));
