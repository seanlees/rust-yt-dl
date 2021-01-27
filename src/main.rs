use rocket::{get, routes, response, Catcher, Request, Error};

use rocket::http::{ContentType, Status};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Cursor;
use std::ffi::OsStr;

use rust_embed::RustEmbed;

use rust_yt_dl::controller::{static_files, index};
use rust_yt_dl::controller::frontend::user;
use rocket::error::LaunchError;
use rocket::response::{Redirect, Responder};

fn redirect_login<'r>(_: LaunchError, r: &'r Request) -> response::Result<'r> {
    Redirect::to("/auth/login").respond_to(r)
}


fn main() {
    let login = Catcher::new(600, redirect_login);
    //let root = Catcher::new(601, redirect_root);

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
        .register(catchers![login])
        .launch();

    //静态资源使用RustEmbed的话，走static_files，下面注释
    //.mount("/static", StaticFiles::from("src/resource")).launch();
}
