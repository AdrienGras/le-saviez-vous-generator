use rocket::local::blocking::Client;

use crate::rocket_builder;

pub fn generate_client() -> Client {
    Client::tracked(rocket_builder::build()).unwrap()
}