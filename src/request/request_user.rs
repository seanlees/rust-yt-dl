use rocket::outcome::IntoOutcome;
use rocket::http::{Cookies, Status};
use rocket::request;
use rocket::response::Redirect;
use serde::{Serialize, Deserialize, ser};
use rocket::request::{FromRequest, Outcome, Request};

use crate::controller::user;
use rocket::outcome::Outcome::{Failure, Success};
use serde::ser::SerializeStruct;


#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub uname: String,
    pub pwd: String,
    pub pwdConfirm: String,
    pub email: String,
}

impl ser::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        let mut s = serializer.serialize_struct("User", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("uname", &self.uname)?;
        s.serialize_field("pwd", &self.pwd)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("is_anonymous", &(self.id == -1))?;
        s.end()
    }
}



