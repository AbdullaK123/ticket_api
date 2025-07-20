use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env::var;
use std::time::Duration;
use anyhow::Result;


pub async fn create_pool() -> Result<PgPool> {
    // grab the connection string
    let db_url = var("DATABASE_URL").expect("Database URI must be set in .env file");

    // config the connection pool
    let pool = 
    
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&db_url)
            .await?;
                                            

    // return the pool wrapped in ok
    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;

    println!("Connection successful!");

    Ok(())
}