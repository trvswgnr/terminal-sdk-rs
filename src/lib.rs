use openapi::apis;
pub use openapi::models;

pub use openapi::apis::configuration::Configuration as Config;

/// Client for the Terminal API
pub struct Client {
    config: Config,
}

impl Client {
    /// Create a new client with the given configuration
    pub fn new(config: Config) -> Self {
        Client { config }
    }

    /// Get the configuration for the client
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

// generated API methods from build.rs
include!(concat!(env!("OUT_DIR"), "/api_methods_gen.rs"));
