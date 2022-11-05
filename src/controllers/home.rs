use rocket_dyn_templates::{Template, context};
use crate::middlewares::quotes;

#[get("/")]
pub fn index() -> Template {
    let quote = quotes::generate();

    Template::render("pages/home/index", context! {quote})
}

#[get("/h/<hash>")]
pub fn with_hash(hash: &str) -> Template {
    Template::render("pages/home/with_hash", context! {
        hash: hash,
        quote: "Le camembert a été inventé en 1953 par Jean-Roger Camembert"
    })
}