use std::io;
use rust_yt_dl::util::file_util;
use std::env;

fn main() -> io::Result<()> {
    println!("{}", file_util::dir_size(".")?);

    let num: u32 = 10;
    println!("{}", (num as f64 / 3.0));

    let key = "ANDROID_SDK_HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key)
    }

    if (env::var_os(key)).is_some(){
        println!("{}: true",key);
    }


    Ok(())
}