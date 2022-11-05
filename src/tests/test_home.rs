#[allow(unused_imports)]
use rocket::{http::Status, local::blocking::Client};
#[allow(unused_imports)]
use crate::{rocket_builder, controllers::home, tests::client_generator};

#[cfg(test)]

#[test]
fn test_index() {
    let client: Client = client_generator::generate_client();
    let response = client.get(uri!(home::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}