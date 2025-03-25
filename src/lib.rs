use openapi::apis;
pub use openapi::models;

pub use openapi::apis::configuration::Configuration as Config;

// generated API methods from build.rs
include!(concat!(env!("OUT_DIR"), "/api_methods_gen.rs"));
