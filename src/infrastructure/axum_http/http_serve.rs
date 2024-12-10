use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use axum::http::Method;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use crate::config::config_model::DotEnvyConfig;
use crate::infrastructure::axum_http::default_routers::{health_check, not_found};
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(not_found)
        .route("/health-check", get(health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(config.server.timeout)))
        .layer(RequestBodyLimitLayer::new((config.server.body_limit * 1024 * 1024).try_into()?))
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]).allow_origin(Any))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(&addr).await?;

    info!("Server is running on: {}", addr);

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
    };

    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl-C signal, shutting down"),
        _ = terminate => info!("Recived terminate signal, shutting down"),
    }
}