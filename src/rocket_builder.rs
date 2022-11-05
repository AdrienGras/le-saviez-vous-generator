use rocket::{Build, Rocket, fs::FileServer};
use rocket_dyn_templates::Template;
use super::controllers::{home, catchers};
use sass_rocket_fairing::SassFairing;

pub fn build() -> Rocket<Build> {
    rocket::build()
    .attach(Template::fairing())
    .attach(SassFairing::default())
    .mount("/", routes![
        home::index,
        home::with_hash
    ])
    .mount("/public", FileServer::from("src/assets"))
    .register("/", catchers![
        catchers::not_found
    ])
}