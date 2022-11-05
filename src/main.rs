#[macro_use] extern crate rocket;

pub mod controllers;
pub mod tests;
pub mod rocket_builder;
pub mod helpers;
pub mod dto;
pub mod middlewares;
pub mod models;
pub mod schema;
pub mod dbal;

#[launch]
fn rocket() -> _ {
    helpers::env::load_env();
    helpers::database::establish_connection();
    rocket_builder::build()
}