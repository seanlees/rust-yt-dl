use crate::request::authenticated_user::AuthenticatedUser;


use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::State;
use crate::config::ConfyConfig;
use crate::util::file_util;

#[get("/")]
pub fn index(_user: AuthenticatedUser,
             cfg: State<ConfyConfig>) -> Template {
    let folder_size = file_util::dir_size(&cfg.store_folder).unwrap_or(0);
    let folder_size_gb = format!("{:.2}{}", (folder_size as f32 / 1024.0 / 1024.0 / 1024.0), "GB");

    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("store_folder", &cfg.store_folder);
    context.insert("store_folder_size", &folder_size_gb);

    Template::render("index", &context)
}

/*#[get("")]
pub fn home(_user: AuthenticatedUser) -> Redirect {
    Redirect::to(uri!(index))
}*/
