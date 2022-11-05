use crate::{dto::quote_dto::QuoteDTO, middlewares::quotes};
use rocket::serde::json::Json;

#[get("/quotes/random")]
pub fn quote_random() -> Json<QuoteDTO> {
    let quote: QuoteDTO = quotes::generate();
    Json(quote)
}

#[get("/quotes/<hash>")]
pub fn quote_by_hash(hash: &str) -> Json<QuoteDTO> {
    let quote: QuoteDTO = quotes::generate_from_hash(hash.into());
    Json(quote)
}