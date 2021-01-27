// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;


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

pub mod view_model{
    pub mod login_form;
}
