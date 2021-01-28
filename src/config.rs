extern crate confy;


#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            name: "Unknown".to_string(),
            comfy: true,
            foo: 42,
        }
    }
}
