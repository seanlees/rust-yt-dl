#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod middleware {
    pub mod counter;
}

pub mod controller {
    pub mod frontend;
    pub mod static_files;
}
