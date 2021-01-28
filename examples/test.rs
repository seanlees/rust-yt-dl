extern crate confy;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct ConfyConfig {
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

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("Config")?;
    let cfg: ConfyConfig = confy::load_path("Config.toml")?;
    println!("The configuration is:");
    println!("{:#?}", cfg);
    Ok(())
}