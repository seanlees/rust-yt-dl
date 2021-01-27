use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::request::request_user::User;
use crate::request::authenticated_user::AuthenticatedUser;

#[get("/")]
pub fn index(_user: AuthenticatedUser) -> Template {
    let context: HashMap<&str, &str> = [("name", "")]
        .iter().cloned().collect();
    Template::render("index", &context)
}