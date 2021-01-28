

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub uname: String,
}