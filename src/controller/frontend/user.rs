use rocket::http::{Cookies, Cookie};
use rocket::response::{Redirect, Flash};
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use crate::request::user_request::User;


#[get("/login")]
pub fn login(mut cookies: Cookies) -> Template {
    let context: HashMap<&str, &str> = [("ctx", "")]
        .iter().cloned().collect();
    Template::render("login", &context)
}


#[get("/auth/login")]
pub fn login1(mut cookies: Cookies) -> std::result::Result<Redirect, Flash<Redirect>> {
    let user = User {
        uid: "".to_string(),
        uname: String::from("abc"),
        pwd: "".to_string(),
        pwdConfirm: "".to_string(),
    };
    cookies.add_private(Cookie::new("sessions_auth", user.uname));
    //println!("{:?}", request.cookies().get_private("token"));
    Ok(Redirect::to(uri!(super::user::login)))
}
