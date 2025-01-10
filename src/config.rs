use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub discord: DiscordConfig,
    pub logging: LoggingConfig
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub codename: String
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String
}

