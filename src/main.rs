#[macro_use] extern crate rocket;

pub mod controllers;
pub mod tests;
pub mod rocket_builder;

#[launch]
fn rocket() -> _ {
    rocket_builder::build()
}