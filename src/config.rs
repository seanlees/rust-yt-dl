extern crate confy;

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    pub login_name: String,
    pub password: String,
    pub store_folder: String,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            login_name: "admin".to_string(),
            password: Uuid::new_v4().to_string(),
            store_folder: ".".to_string(),
        }
    }
}
