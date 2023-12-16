use axum::{extract, http, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Quote {
    id: Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl Quote {
    pub fn new(book: String, quote: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            book,
            quote,
            inserted_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(http::StatusCode, Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.inserted_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
