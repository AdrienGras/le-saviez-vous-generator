use rocket::local::blocking::Client;
use crate::{rocket_builder, controllers::home};
use rocket::http::Status;

#[cfg(test)]

#[test]
fn test_index() {
    let rocket = rocket_builder::build();
    let client = Client::tracked(rocket).unwrap();
    let response = client.get(uri!(home::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}