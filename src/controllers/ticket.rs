// src/controllers/ticket.rs
use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json
};
use serde::Deserialize;
use uuid::Uuid;
use anyhow::Result;
use tracing::{info, warn, error, debug}; // ← Add this
use crate::{
     models::{CreateTicketRequest, UpdateTicketRequest}, 
    services::TicketService, 
    views::TicketResponse
};

#[derive(Deserialize, Debug)] // ← Add Debug
pub struct SearchQuery {
    pub q: Option<String>,
    pub status: Option<String>
}

// creation handler
pub async fn create_ticket(
    State(service): State<TicketService>,
    Json(payload): Json<CreateTicketRequest>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    info!(title = %payload.title, status = %payload.status, "Creating new ticket");
    
    match service.create(payload).await {
        Ok(ticket) => {
            info!(ticket_id = %ticket.id, "Successfully created ticket");
            Ok((StatusCode::CREATED, Json(ticket)))
        },
        Err(e) => {
            error!(error = %e, "Failed to create ticket");
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

// update handler
pub async fn update_ticket(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTicketRequest>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    info!(ticket_id = %id, "Updating ticket");
    debug!(update_payload = ?payload, "Update payload details");
    
    match service.update(id, payload).await {
        Some(ticket) => {
            info!(ticket_id = %ticket.id, "Successfully updated ticket");
            Ok((StatusCode::OK, Json(ticket)))
        },
        None => {
            warn!(ticket_id = %id, "Ticket not found for update");
            Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()))
        }
    }
}

// delete handler
pub async fn delete_ticket(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, (StatusCode, String)> {
    info!(ticket_id = %id, "Deleting ticket");
    
    match service.delete(id).await {
        Ok(true) => {
            info!(ticket_id = %id, "Successfully deleted ticket");
            Ok(StatusCode::NO_CONTENT)
        },
        Ok(false) => {
            warn!(ticket_id = %id, "Ticket not found for deletion");
            Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()))
        },
        Err(e) => {
            error!(ticket_id = %id, error = %e, "Failed to delete ticket");
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

// get by id handler
pub async fn get_ticket_by_id(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    debug!(ticket_id = %id, "Fetching ticket by ID");
    
    match service.get_by_id(id).await {
        Some(ticket) => {
            debug!(ticket_id = %ticket.id, "Successfully found ticket");
            Ok((StatusCode::OK, Json(ticket)))
        },
        None => {
            warn!(ticket_id = %id, "Ticket not found");
            Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()))
        }
    }
}

// get handler
pub async fn get_tickets(
    State(service): State<TicketService>,
    Query(params): Query<SearchQuery>,
) -> Result<(StatusCode, Json<Vec<TicketResponse>>), (StatusCode, String)> {
    info!(search_params = ?params, "Fetching tickets");

    let tickets = match (params.q.as_deref(), params.status.as_deref()) {
        (Some(q), None) => {
            info!(search_term = %q, "Searching tickets by text");
            service.get_by_text_search(q).await
        },
        (None, Some(status)) => {
            info!(status = %status, "Filtering tickets by status");
            service.get_by_status(status.to_string()).await
        },
        (None, None) => {
            info!("Fetching all tickets");
            service.get_all().await
        },
        (Some(_), Some(_)) => {
            warn!("Invalid request: both search and filter provided");
            return Err((StatusCode::BAD_REQUEST, "Can not filter and search at the same time".to_string()))
        }
    };

    match tickets {
        Ok(tickets) => {
            info!(count = tickets.len(), "Successfully retrieved tickets");
            Ok((StatusCode::OK, Json(tickets)))
        },
        Err(e) => {
            error!(error = %e, "Failed to retrieve tickets");
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

// Router stays the same
pub fn init_controller() -> Router<TicketService> {
    Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/{id}", get(get_ticket_by_id))
        .route("/tickets/{id}", put(update_ticket))
        .route("/tickets/{id}", delete(delete_ticket))
}