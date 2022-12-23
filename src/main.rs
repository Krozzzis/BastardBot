mod config;

use telegram_system::TelegramSystem;
use timetable_system::TimeTableProvider;
use config::BastardConfig;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config_path = "config.toml";
    let config = match BastardConfig::from_file(config_path) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("ConfigError: {}", err.to_string());
            std::process::exit(1);
        }
    };

    let ttp = TimeTableProvider::new();
    let tg = TelegramSystem::new(ttp);
    tg.run(config.telegram_token.clone()).await;
}
