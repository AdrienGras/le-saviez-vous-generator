use rocket_dyn_templates::{Template, context};
use crate::{middlewares::quotes, dto::quote_dto::QuoteDTO};

#[get("/")]
pub fn index() -> Template {
    let quote: QuoteDTO = quotes::generate();

    Template::render("pages/home/index", context! {
        quote
    })
}

#[get("/h/<hash>")]
pub fn with_hash(hash: &str) -> Template {
    let quote: QuoteDTO = quotes::generate_from_hash(hash.into());

    Template::render("pages/home/with_hash", context! {
        quote
    })
}