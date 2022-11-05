use config::Config;

#[derive(serde::Deserialize)]
pub struct RedisConfiguration {
    pub host: String,
    pub port: u16
}

#[derive(serde::Deserialize)]
pub struct LoggerConfiguration {
    pub write_style: String,
    pub filters: String
}

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub redis: RedisConfiguration,
    pub logger: LoggerConfiguration
}

pub fn load_config() -> Configuration {
    Config::builder()
        .add_source(config::File::with_name("cfg/defaults.toml"))
        .add_source(config::File::with_name("cfg/config.toml").required(false))
        .add_source(config::Environment::with_prefix("tblt").separator("_"))
        .build()
        .expect("Failed to load configuration")
        .try_deserialize()
        .expect("Failed to deserialize configuration")
}
