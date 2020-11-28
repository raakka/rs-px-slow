pub mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config{
        pub api_addr: String,
    }

    impl Config {
        pub fn from_env() -> Result<self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}
