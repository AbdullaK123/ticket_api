mod models;
mod views;
mod config;
mod repositories;
mod services;
mod controllers;

use anyhow::Result;
use tokio::net::TcpListener;
use crate::config::database::create_pool;
use crate::controllers::init_app;
use crate::services::TicketService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let pool = create_pool().await?;

    let ticket_service = TicketService::new(pool);

    let listener = TcpListener::bind("localhost:3000").await?;

    let app = init_app().with_state(ticket_service);

    axum::serve(listener, app).await?;

    println!("🚀 Server running on http://localhost:3000");

    Ok(())
}