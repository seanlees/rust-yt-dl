extern crate confy;
#[macro_use]
extern crate serde_derive;

use rust_yt_dl::config::ConfyConfig;

fn main() -> Result<(), confy::ConfyError> {
    //let cfg: ConfyConfig = confy::load("Config")?;
    let cfg: ConfyConfig = confy::load_path("Config.toml")?;
    println!("The configuration is:");
    println!("{:#?}", cfg);

    let a = "ss";
    let b = format!("{0}{1}", a, "b");
    println!("{:?}", b);

    let b = "ss";
    let c = b.to_string();
    let d = &c;
    println!("b=c:{},b=d:{}", b == c, b == d);

    Ok(()) //注意：confy需要main方法有返回值
}
