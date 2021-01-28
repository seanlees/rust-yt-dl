use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

use crate::request::authenticated_user::AnonymousUser;
use crate::request::request_user::User;
use crate::view_model::login_form::LoginForm;
use crate::view_model::login_resp::LoginResponse;

use serde::{Deserialize, ser, Serializer};
use serde::Serialize;
use serde_json::json;
use serde::ser::SerializeStruct;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct LoginRespJson<'a> {
    code: u8,
    msg: &'a str,
    user: LoginResponse,
}


#[get("/login")]
pub fn login(user: AnonymousUser, mut cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.insert("user", &user);

    Template::render("login", &context)
}


#[post("/login", data = "<form>")]
pub fn authenticate(user: AnonymousUser,
                    form: Form<LoginForm>,
                    mut cookies: Cookies)
                    -> Json<LoginRespJson> {
    let noLogin: bool = cookies.get("sessions_auth").is_none();

    if !noLogin {
        let resp = LoginRespJson {
            code: 99,
            msg: "未成功登录",
            user: LoginResponse {
                id: 0,
                uname: "".to_string(),
            },
        };
        return Json(resp);
    }

    return Json(LoginRespJson {
        code: 0,
        msg: "登录成功",
        user: LoginResponse {
            id: 0,
            uname: "".to_string(),
        },
    });
}
