use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct QuoteDTO {
    pub subject: String,
    pub verb: String,
    pub description: String,
    pub complete_quote: String,
    pub hash: String
}