// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

//声明再这里，子模块无需再次生命
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate diesel;

//use rocket_contrib::databases::diesel;

#[database("my_db")]
pub struct DbConn(diesel::SqliteConnection);

pub mod middleware {
    pub mod counter;
}

pub mod controller {
    pub mod frontend {}

    pub mod index;
    pub mod login;
    pub mod static_files;
    pub mod list;
}

pub mod request {
    pub mod authenticated_user;
    pub mod request_user;
}

pub mod view_model {
    pub mod login_form;
    pub mod login_resp;
}

pub mod util {
    pub mod file_util;
    pub mod template_util;
}

pub mod config;
pub mod dict;
pub mod models;
pub mod schema;
