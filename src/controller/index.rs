use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::request::user_request::User;

#[get("/")]
pub fn index(user: User) -> Template {
    let context: HashMap<&str, &str> = [("name", "")]
        .iter().cloned().collect();
    Template::render("index", &context)
}