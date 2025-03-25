use openapi::apis;
pub use openapi::apis::configuration::Configuration as Config;
pub use openapi::models;

/// Represents a client for the [Terminal API]
/// 
/// [Terminal API]: https://www.terminal.shop/api
#[derive(Debug, Clone)]
pub struct Client {
    pub config: Config,
}

impl Default for Client {
    /// Creates a new client with the default configuration
    fn default() -> Self {
        Self::new(Config::default())
    }
}

// generated client from build.rs
include!(concat!(env!("OUT_DIR"), "/api_methods_gen.rs"));
