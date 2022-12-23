use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BastardConfig {
    pub telegram_token: String,
}

impl BastardConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(path)?;
        let config: BastardConfig = toml::from_str(file.as_str())?;
        Ok(config)
    }
}
