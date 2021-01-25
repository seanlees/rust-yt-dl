// Rocket用到的rust的nightly的特性
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{get, Rocket, routes};

#[get("/")]
fn hello() -> &'static str {
    return "hello rocket";
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
