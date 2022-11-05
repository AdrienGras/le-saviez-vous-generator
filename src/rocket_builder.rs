use rocket::{Build, Rocket, fs::FileServer};
use rocket_dyn_templates::Template;
use super::controllers::{home, catchers, api};
use sass_rocket_fairing::SassFairing;

pub fn build() -> Rocket<Build> {
    rocket::build()
    .attach(Template::fairing())
    .attach(SassFairing::default())
    .mount("/", routes![
        home::index,
        home::with_hash,
        home::about
    ])
    .mount("/api", routes![
        api::quote_random,
        api::quote_by_hash
    ])
    .mount("/public", FileServer::from("src/assets"))
    .register("/", catchers![
        catchers::not_found
    ])
}