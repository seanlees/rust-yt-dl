
#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub uname: String,
    pub password: String,
}