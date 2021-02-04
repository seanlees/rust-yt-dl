#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;

use rocket::{Catcher, get, Request, response, routes};
use rocket::error::LaunchError;
use rocket::http::{ContentType, Status};
use rocket::response::{content, Redirect, Responder};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use rust_yt_dl::config::ConfyConfig;
use rust_yt_dl::controller::*;
use rust_yt_dl::controller::login;
use rust_yt_dl::DbConn;

#[catch(401)]
fn redirect_login(_req: &Request) -> Template {
    let context: HashMap<&str, &str> = [("name", "")].iter().cloned().collect();
    Template::render("login", &context)
}

#[catch(599)]
fn redirect_root(_req: &Request) -> Template {
    let context: HashMap<&str, &str> = [("name", "")].iter().cloned().collect();
    Template::render("index", &context)
}

fn main() -> Result<(), confy::ConfyError> {
    //自定义配置参数
    let cfg: ConfyConfig = confy::load_path("Config.toml")?;

    let rocket = rocket::ignite();
    let context_path = rocket
        .config()
        .get_str("context_path")
        .unwrap_or("/")
        .to_string();

    rocket
        .manage(cfg)
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .mount(
            &context_path,
            routes![
                //static_files::haddle,  //静态资源打包成exe时使用这个
                index::index,
                login::login,
                login::authenticate,
                login::logout,
                list::list,
            ],
        )
        .register(catchers![redirect_login, redirect_root])
        //静态资源使用RustEmbed的话，走static_files，下面注释
        .mount("/static", StaticFiles::from("src/resource"))
        .launch();

    Ok(())
}
