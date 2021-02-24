use std::io;
use rust_yt_dl::util::file_util;

fn main() -> io::Result<()> {
    println!("{}", file_util::dir_size(".")?);

    let num: u32 = 10;
    println!("{}", (num as f64 / 3.0));

    Ok(())
}