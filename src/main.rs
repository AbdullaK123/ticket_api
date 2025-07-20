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
use tracing::info;
use crate::config::database::create_pool;
use crate::controllers::init_app;
use crate::services::TicketService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 Starting ticket application");

    // Add more detailed error handling here
    let pool = match create_pool().await {
        Ok(pool) => {
            info!("✅ Database connection pool created");
            pool
        },
        Err(e) => {
            tracing::error!("❌ Failed to create database pool: {}", e);
            return Err(e);
        }
    };

    // Try to run migrations with better error handling
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => info!("✅ Database migrations completed"),
        Err(e) => {
            tracing::error!("❌ Failed to run migrations: {}", e);
            return Err(e.into());
        }
    }

    let ticket_service = TicketService::new(pool);
    info!("✅ Ticket service initialized");

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("🌐 Server listening on http://0.0.0.0:3000");

    let app = init_app()
        .layer(TraceLayer::new_for_http())
        .with_state(ticket_service);

    info!("🚀 Server starting...");
    axum::serve(listener, app).await?;

    Ok(())
}