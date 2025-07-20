use anyhow::Result;
use uuid::Uuid;
use sqlx::PgPool;
use crate::repositories::TicketRepository;
use crate::views::TicketResponse;
use crate::models::{ 
     CreateTicketRequest, 
     UpdateTicketRequest
};

#[derive(Clone)]
pub struct TicketService {
    repo: TicketRepository
}

impl TicketService {

    pub fn new(pool: PgPool) -> Self {

        let repo = TicketRepository::new(pool);

        Self { repo }
    }

    // search tickets by keywords or phrases in title or description
    pub async fn get_by_text_search(&self, search_term: &str) -> Result<Vec<TicketResponse>> {
        let tickets = self.repo.get_all().await?;

        let ticket_responses = 

            tickets
                .iter()
                .filter(|&ticket| {
                    ticket.title.contains(search_term) || 
                    ticket.description.contains(search_term)
                })
                .map(|ticket| {
                    TicketResponse::from(ticket)
                })
                .collect();
        
        Ok(ticket_responses)
    }

    // get all tickets by status
    pub async fn get_by_status(&self, status: String) -> Result<Vec<TicketResponse>> {
        let tickets = self.repo.get_all().await?;

        let ticket_responses = 

            tickets
                .iter()
                .filter(|&ticket| {
                     ticket.status == status
                })
                .map(|ticket| {
                    TicketResponse::from(ticket)
                })
                .collect();

        Ok(ticket_responses)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Option<TicketResponse> {
        let ticket = self.repo.get_by_id(id).await.ok()?;

        if let Some(ticket) = ticket {
            return Some(TicketResponse::from(ticket));
        } else {
            None
        }

    }

    // update a ticket
    pub async fn update(&self, id: Uuid, payload: UpdateTicketRequest) -> Option<TicketResponse> {

        let ticket = self.repo.update(id, payload).await.ok()?;

        if let Some(ticket) = ticket {
            return Some(TicketResponse::from(ticket));
        } else {
            None
        }

    }

    // get all
    pub async fn get_all(&self) -> Result<Vec<TicketResponse>> {

        let tickets = self.repo.get_all().await?;

        let responses = tickets.into_iter().map(TicketResponse::from).collect();

        Ok(responses)
    }

    // delete a ticket
    pub async fn delete(&self, id: Uuid) -> Result<bool> {

        let result = self.repo.delete(id).await?;

        Ok(result)
    }

    // create a ticket
    pub async fn create(&self, payload: CreateTicketRequest) -> Result<TicketResponse> {
        let ticket = self.repo.create(payload).await?;

        Ok(TicketResponse::from(ticket))
    }

}