mod config;
mod db;
mod handlers;
mod helper;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    tracing::info!("Starting {}...", config.app_name);

    let pool = db::init_db_pool(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");

    tracing::info!("Connected to PostgreSQL database successfully.");

    let state = Arc::new(AppState { pool, config: config.clone() });

    std::fs::create_dir_all(&config.upload_path).ok();

    let app = Router::new()
        .route("/", get(handlers::handler::get_base))
        .route("/health", get(handlers::handler::get_health))
        .route("/upload", post(handlers::handler::post_upload))
        .nest_service("/uploads", ServeDir::new(&config.upload_path))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.app_host, config.app_port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
