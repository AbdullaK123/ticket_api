pub mod ticket;

use axum::Router;
use crate::services::TicketService;


pub fn init_app() -> Router<TicketService> {
    Router::new()
        .merge(ticket::init_controller())
}