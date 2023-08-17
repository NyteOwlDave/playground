
use std::env;
use std::env::VarError;

pub fn getenv(name: &str) -> Result<String, VarError> {
    env::var(name)
}
