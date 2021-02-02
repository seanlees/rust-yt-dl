use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;

use crate::request::authenticated_user::{AnonymousUser, AuthenticatedUser};
use crate::request::request_user::User;
use crate::view_model::login_form::LoginForm;
use crate::view_model::login_resp::LoginResponse;

use crate::config::ConfyConfig;
use rocket::State;
use rocket_contrib::json::Json;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::{ser, Deserialize, Serializer};
use serde_json::json;
use serde::ser::SerializeStruct;
use rocket_contrib::json::Json;
use rocket::State;
use crate::config::ConfyConfig;
use crate::DbConn;

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
pub fn authenticate<'a>(
    user: AnonymousUser,
    cfg: State<ConfyConfig>,
    form: Form<LoginForm>,
    mut cookies: Cookies,
) -> Json<LoginRespJson<'a>> {
    let noLogin: bool = cookies.get("sessions_auth").is_none();

    if !noLogin {
        let resp = LoginRespJson {
            code: 0,
            msg: "登录成功",
            user: LoginResponse {
                id: 0,
                uname: "".to_string(),
            },
        };
        return Json(resp);
    }

    let req_uname = &form.0.uname;
    let req_pwd = &form.0.password;

    if &req_uname != &&cfg.login_name || &req_pwd != &&cfg.password {
        let resp = LoginRespJson {
            code: 99,
            msg: "登录失败,用户名或密码错误!",
            user: LoginResponse {
                id: 0,
                uname: "".to_string(),
            },
        };
        return Json(resp);
    }

    let user = &User {
        id: 0,
        uname: req_uname.to_string(),
        pwd: "".to_string(),
        email: "".to_string(),
    };

    let authSession =
        Cookie::build("sessions_auth".to_owned(), serde_json::to_string(user).unwrap())
            .path("/")
            .finish();

    cookies.add_private(authSession);

    return Json(LoginRespJson {
        code: 0,
        msg: "登录成功",
        user: LoginResponse {
            id: 0,
            uname: "".to_string(),
        },
    });
}

#[get("/logout")]
pub fn logout<'a>(user: AuthenticatedUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("sessions_auth"));
    Redirect::to(uri!(login))
}
