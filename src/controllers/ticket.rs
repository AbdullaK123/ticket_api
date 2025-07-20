use axum::{
    Router,
    routing::{
        get,
        post,
        put,
        delete
    },
    extract::{
        Path, Query, State
    },
    http::StatusCode,
    response::Json
};
use serde::Deserialize;
use uuid::Uuid;
use anyhow::Result;
use crate::{
     models::{
        CreateTicketRequest,
        UpdateTicketRequest
    }, 
    services::TicketService, 
    views::TicketResponse
};


#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub status: Option<String>
}


// creation handler
pub async fn create_ticket(
    State(service): State<TicketService>,
    Json(payload): Json<CreateTicketRequest>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    match service.create(payload).await {
        Ok(ticket) => Ok((StatusCode::CREATED, Json(ticket))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}


// update handler
pub async fn update_ticket(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTicketRequest>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    match service.update(id, payload).await {
        Some(ticket) => Ok((StatusCode::OK, Json(ticket))),
        None => Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()))
    }
}


// delete handler
pub async fn delete_ticket(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, (StatusCode, String)> {
    match service.delete(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Ticket not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

// get by id handler
pub async fn get_ticket_by_id(
    State(service): State<TicketService>,
    Path(id): Path<Uuid>
) -> Result<(StatusCode, Json<TicketResponse>), (StatusCode, String)> {
    match service.get_by_id(id).await {
        Some(ticket) => Ok((StatusCode::OK, Json(ticket))),
        None => Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()))
    }
}


// get handler
pub async fn get_tickets(
    State(service): State<TicketService>,
    Query(params): Query<SearchQuery>,
) -> Result<(StatusCode, Json<Vec<TicketResponse>>), (StatusCode, String)> {

    let tickets = match (params.q, params.status) {
        
        (Some(q), None) => service.get_by_text_search(q.as_str()).await,

        (None, Some(status)) => service.get_by_status(status).await,

        (None, None) => service.get_all().await,

        (Some(_), Some(_)) => {
            return Err((StatusCode::BAD_REQUEST, "Can not filter and search at the same time".to_string()))
        }

    };

    match tickets {
        Ok(tickets) => Ok((StatusCode::OK, Json(tickets))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

pub fn init_controller() -> Router<TicketService> {
    Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/{id}", get(get_ticket_by_id))
        .route("/tickets/{id}", put(update_ticket))
        .route("/tickets/{id}", delete(delete_ticket))
}