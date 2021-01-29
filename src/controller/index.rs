use crate::request::authenticated_user::AuthenticatedUser;
use crate::request::request_user::User;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index(_user: AuthenticatedUser) -> Template {
    let context: HashMap<&str, &str> = [("name", "")].iter().cloned().collect();
    Template::render("index", &context)
}

/*#[get("")]
pub fn home(_user: AuthenticatedUser) -> Redirect {
    Redirect::to(uri!(index))
}*/
