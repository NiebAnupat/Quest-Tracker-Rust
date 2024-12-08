use tracing::{error, info};
use quests_tracker::config::config_loader;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let dotenvy_env = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load env: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Loaded env: {:?}", dotenvy_env);
}
