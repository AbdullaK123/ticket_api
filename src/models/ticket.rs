use sqlx::{FromRow};
use serde::{Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;


// rust struct sqlx will map to rust structs
#[derive(Debug, FromRow, Clone)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


// request body for post end point
#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
    pub status: String
}

// request body for put end point
#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>
}