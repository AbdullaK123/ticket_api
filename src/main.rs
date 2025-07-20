// src/main.rs
mod models;
mod views;
mod config;
mod repositories;
mod services;
mod controllers;

use anyhow::Result;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info; // ← Add this
use crate::config::database::create_pool;
use crate::controllers::init_app;
use crate::services::TicketService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Starting ticket application");

    let pool = create_pool().await?;
    info!("✅ Database connection pool created");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("✅ Database migrations completed");

    let ticket_service = TicketService::new(pool);
    info!("✅ Ticket service initialized");

    let listener = TcpListener::bind("localhost:3000").await?;
    info!("🌐 Server listening on http://localhost:3000");

    let app = init_app()
        .layer(TraceLayer::new_for_http())
        .with_state(ticket_service);

    info!("🚀 Server starting...");
    axum::serve(listener, app).await?;

    Ok(())
}