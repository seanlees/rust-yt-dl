// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{get, routes, response};

use rocket::http::{ContentType, Status};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Cursor;
use std::ffi::OsStr;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resource/"]
struct Asset;

#[get("/static/<file..>")]
fn static_files<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    Asset::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
            let content_type = ContentType::from_extension(ext).ok_or_else(|| Status::new(400, "Could not get file content type"))?;
            response::Response::build().header(content_type).sized_body(Cursor::new(d)).ok()
        },
    )
}


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
        .mount(&context_path, routes![static_files,hello,index]).launch();
    //静态资源使用RustEmbed的话，走staticFiles，下面注释
    //.mount("/static", StaticFiles::from("src/resource")).launch();
}
