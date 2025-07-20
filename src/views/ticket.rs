use serde::Serialize;
use uuid::Uuid;
use crate::models::Ticket;
use chrono::Utc;


#[derive(Serialize, Debug)]
pub struct TicketResponse {
    pub id: Uuid,
    pub title: String,
    pub status_display: String,
    pub days_old: i32,
    pub priority: String,
    pub status_color: String
}

impl From<Ticket> for TicketResponse {
    fn from(value: Ticket) -> Self {

        let days_old = ( Utc::now().naive_utc() - value.created_at).num_days() as i32;

        TicketResponse { 
            id: value.id,
            title: value.title, 
            status_display: format!("Status: {}", value.status), 
            days_old: days_old,
            priority: if days_old > 7 {"High".to_string()} else {"Normal".to_string()},
            status_color: match value.status.as_str() {
                "To Do" => "#red".to_string(),
                "In Progress" => "#yellow".to_string(),
                _ => "#green".to_string()
            }
         }
    }
}

impl From<&Ticket> for TicketResponse {
    fn from(value: &Ticket) -> Self {
        Self::from(value.clone())
    }
}