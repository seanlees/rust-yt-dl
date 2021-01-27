use rocket::http::{Cookies, Cookie};
use rocket::response::{Redirect, Flash};
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use crate::request::request_user::User;
use crate::request::authenticated_user::AnonymousUser;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;

#[derive(Serialize)]
struct InvalidFormMessage<'a> {
    code: &'a u16,
    msg: &'a str,
}

#[get("/login")]
pub fn login(user: AnonymousUser, mut cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.insert("user", &user);

    Template::render("login", &context)
}


#[post("/login"), data = "<form>"]
pub fn authenticate(user: AnonymousUser,
                    form: Form,
                    mut cookies: Cookies)
                    -> std::result::Result<Redirect, Flash<Redirect>> {
    cookies.add_private(Cookie::new("sessions_auth", "123"));
    //println!("{:?}", request.cookies().get_private("token"));
    Ok(Redirect::to(uri!(super::user::login)))
}
