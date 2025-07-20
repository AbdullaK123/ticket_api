use sqlx::postgres::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{
    CreateTicketRequest, 
    Ticket, 
    UpdateTicketRequest
};

#[derive(Clone)]
pub struct TicketRepository {
    pool: PgPool 
}

impl TicketRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, payload: CreateTicketRequest) -> Result<Ticket> {
        let ticket = sqlx::query_as!(
            Ticket,
            "
            INSERT INTO tickets (title, description, status)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                title,
                description,
                status,
                created_at,
                updated_at
            ",
            payload.title, 
            payload.description, 
            payload.status
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ticket)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Ticket>> {
        let ticket = sqlx::query_as!(
            Ticket,
            "
            SELECT * 
            FROM tickets
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(ticket)
    }

    pub async fn get_all(&self) -> Result<Vec<Ticket>> {
        let tickets = sqlx::query_as!(
            Ticket,
            "
            SELECT *
            FROM Tickets
            "
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    pub async fn update(&self, id: Uuid, payload: UpdateTicketRequest) -> Result<Option<Ticket>> {
        let ticket = sqlx::query_as!(
            Ticket,
            "
            UPDATE tickets
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                id,
                title,
                description,
                status,
                created_at,
                updated_at
            ",
            id,
            payload.title,
            payload.description,
            payload.status
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(ticket)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            "
            DELETE FROM tickets
            WHERE id = $1
            ",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

}