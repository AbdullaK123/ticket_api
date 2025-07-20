mod models;
mod views;
mod config;
mod repositories;
mod services;
mod controllers;

use anyhow::Result;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use crate::config::database::create_pool;
use crate::controllers::init_app;
use crate::services::TicketService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = create_pool().await?;

    let ticket_service = TicketService::new(pool);

    let listener = TcpListener::bind("localhost:3000").await?;

    let app = 
        
        init_app()
            .layer(TraceLayer::new_for_http())
            .with_state(ticket_service);

    axum::serve(listener, app).await?;

    tracing::info!("🚀 Server running on http://localhost:3000");

    Ok(())
}