use std::collections::HashMap;

use rocket_contrib::templates::Template;

use crate::dict;
use crate::request::authenticated_user::AuthenticatedUser;
use crate::dict::listTypeVideo;

#[get("/list?<typ>")]
pub fn list(_user: AuthenticatedUser, typ: Option<String>) -> Template {
    let context: HashMap<&str, &str> = [("name", "")].iter().cloned().collect();
    let ttype = &typ.unwrap_or(listTypeVideo.to_owned());

    if &ttype == &listTypeVideo {
        Template::render("list", &context)
    } else {
        Template::render("list_a", &context)
    }
}
