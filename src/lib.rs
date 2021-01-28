// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]

//声明再这里，子模块无需再次生命
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;


pub mod middleware {
    pub mod counter;
}

pub mod controller {
    pub mod frontend {}

    pub mod static_files;
    pub mod index;
    pub mod user;
}

pub mod request {
    pub mod request_user;
    pub mod authenticated_user;
}

pub mod view_model {
    pub mod login_form;
    pub mod login_resp;
}

pub mod config;