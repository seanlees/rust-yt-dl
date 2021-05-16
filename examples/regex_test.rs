extern crate regex;

use std::{thread, env};

use regex::Regex;
use std::path::{PathBuf, Path};

macro_rules! regex (
    ($s:expr) => (regex::Regex::new($s).unwrap());
);

fn main() {
    let REGEX_NAME: regex::Regex = regex!(r"(.*)\.([a-zA-Z0-9]+)\z");
    //let REGEX_NAME: regex::Regex = Regex::new("(.*)\\.([a-zA-Z0-9]+)\\z").unwrap();
    let c = REGEX_NAME.
        captures("中共脱贫表彰大会的自洽与逻辑_国资洽购苏宁易购, 国进民退(字幕)_王剑每日观察_20210225.mp4");
    println!("{}", c.unwrap()[1].to_string());

    let c = REGEX_NAME.
        captures("中共脱贫表彰大会的自洽与逻辑_国资洽购苏宁易购, 国进民退(字幕)_王剑每日观察_20210225.mp4");
    println!("{}", c.unwrap()[1].to_string());

    let a = "string";
    let new_thread = thread::spawn(move || {
        test(a);
    });
    new_thread.join().unwrap();

    let target = env::var("TARGET").unwrap_or_else(|_| String::new());
    let host = env::var("HOST").unwrap_or_else(|_| String::new());
    println!("target:{}, host:{}", target, host);

    let r = Regex::new(r"\A(https://|http://)?(www\.|m\.)youtube\.(com|de)/.*watch\?v=[a-zA-Z0-9_-]+").unwrap();
    let v = "https://www.youtube.com/watch?v=nXeeNE0jDNs";
    println!("{}", r.captures(&v).unwrap()[0].to_string());
    for (i, c) in r.captures_iter(&v).enumerate() {
        for j in 0..c.len() {
            println!("group {},{} : {}", i, j, &c[j]);
        }
    }
}


fn test(a: &str) {
    println!("finished startup:{}",a);
}