use rocket::response::Redirect;

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to(uri!(super::home::index))
}