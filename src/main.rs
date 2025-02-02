use std::sync::Arc;
use tracing::{error, info};
use quests_tracker::config::config_loader;
use quests_tracker::infrastructure::axum_http::http_serve::start;
use quests_tracker::infrastructure::postgres::postgres_connection;

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

    info!("Loaded env");

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Established database connection");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await.expect("Failed to start server");
}
