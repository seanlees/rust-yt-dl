use rocket::outcome::IntoOutcome;
use rocket::http::{Cookies, Status};
use rocket::request;
use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome, Request};

use crate::controller::frontend::user;
use rocket::outcome::Outcome::{Failure, Success};


#[derive(Debug)]
pub struct User {
    pub uid: String,
    pub uname: String,
    pub pwd: String,
    pub pwdConfirm: String,
}

#[derive(Debug)]
pub enum UserError {
    Invalid,
}

/*impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserError;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|val| User { uid: "".to_string(), uname: val, pwd: "".to_string(), pwdConfirm: "".to_string() })
            .or_forward(Redirect::to(uri!(user::login)));
    }
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_err() {
                return Failure((Status::raw(600),()));
            }

            return Success(AuthenticatedUser(user.unwrap()));
        }

        Failure((Status::raw(600), UserError.Invalid))
    }
}

