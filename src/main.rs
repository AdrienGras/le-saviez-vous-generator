#[macro_use] extern crate rocket;

use controllers::catchers::not_found;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use sass_rocket_fairing::SassFairing;

pub mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .attach(SassFairing::default())
    .mount("/", routes![
        controllers::home::index,
        controllers::home::with_hash
    ])
    .mount("/public", FileServer::from("src/assets"))
    .register("/", catchers![
        not_found
    ])
}