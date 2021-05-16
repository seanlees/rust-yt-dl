use serde::{Deserialize, ser};
use serde::ser::SerializeStruct;



#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub uname: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pwd: String,
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
