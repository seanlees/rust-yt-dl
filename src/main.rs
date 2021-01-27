#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, response, Catcher, Request};

use rocket::http::{ContentType, Status};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::error::LaunchError;
use rocket::response::{Redirect, Responder, content};

use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Cursor;
use std::ffi::OsStr;

use rust_embed::RustEmbed;

use rust_yt_dl::controller::{static_files, index};
use rust_yt_dl::controller::user;


#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[catch(401)]
fn redirect_login(_req: &Request) -> Template {
    let context: HashMap<&str, &str> = [("name", "")]
        .iter().cloned().collect();
    Template::render("login", context)
}

#[catch(599)]
fn redirect_root(_req: &Request) -> Template {
    let context: HashMap<&str, &str> = [("name", "")]
        .iter().cloned().collect();
    Template::render("index", context)
}

fn main() {
    let rocket = rocket::ignite();
    let context_path = rocket.config().get_str("context_path").unwrap_or("/").to_string();

    rocket
        .attach(Template::fairing())
        .mount(&context_path,
               routes![
               static_files::haddle,
               index::index,
               user::login,
               user::login1
               ])
        .register(catchers![redirect_login])
        .launch();

    //静态资源使用RustEmbed的话，走static_files，下面注释
    //.mount("/static", StaticFiles::from("src/resource")).launch();
}
