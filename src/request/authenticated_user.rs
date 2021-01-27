
use crate::request::request_user::User;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, request};
use rocket::outcome::Outcome::{Failure, Success};
use rocket::http::Status;
use serde::Deserialize;
use serde::Serialize;


pub struct AuthenticatedUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_err() {
                return Failure((Status::raw(401), ()));
            }

            return Success(AuthenticatedUser(user.unwrap()));
        }

        Failure((Status::raw(401), ()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AnonymousUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AnonymousUser, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_ok() {
                return Failure((Status::raw(599), ()));
            }
        }

        Success(AnonymousUser(User {
            id: -1,
            uname: "".to_owned(),
            email: "".to_owned(),
            pwd: "".to_owned(),
            pwdConfirm: "".to_string()
        }))
    }
}
