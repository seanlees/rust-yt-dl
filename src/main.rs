// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, response};

use rocket::http::{ContentType, Status};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Cursor;
use std::ffi::OsStr;

use rust_embed::RustEmbed;

use rust_yt_dl::controller::static_files;

#[get("/hello")]
fn hello() -> &'static str {
    return "hello rocket";
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "haofeng")]
        .iter().cloned().collect();
    Template::render("login", &context)
}

fn main() {
    let rocket = rocket::ignite();
    let context_path = rocket.config().get_str("context_path").unwrap_or("/").to_string();

    rocket.attach(Template::fairing())
        .mount(&context_path, routes![static_files::haddle,hello,index]).launch();
    //静态资源使用RustEmbed的话，走static_files，下面注释
    //.mount("/static", StaticFiles::from("src/resource")).launch();
}
