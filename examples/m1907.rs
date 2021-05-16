use std::{io, ascii};
use rust_yt_dl::util::file_util;
use std::env;
use chrono::{Local, TimeZone, Datelike};
use core::mem;
use std::ops::Add;

#[test]
fn sign() {
    let dt = Local::now();
    let currentTimestamp = dt.timestamp_millis();
    println!("dt: {}", currentTimestamp);
    let offset = dt.offset().utc_minus_local() * 1000;
    println!("offset:{}", offset);

    let ctm = currentTimestamp + offset as i64 + 3600000 * 8;
    let dt2 = Local.timestamp_millis(ctm);
    let p = dt2.day() + 9 + 9 ^ 10;
    println!("p:{}", p);

    let p = p.to_string();

    let digest = md5::compute(p);
    let md5 = format!("{:x}", digest);
    println!("md5 first:{}", md5);

    println!("digest 2 first:{}", &md5[0..10]);
    let finalDigest = md5::compute(&md5[0..10]);
    let finalMd5 = format!("{:x}", finalDigest);
    println!("md5 final:{}", finalMd5);

    let weekday = dt2.weekday().num_days_from_sunday();
    println!("arg2:{}", weekday);
    let slig = weekday + 11397;

    let mut url = String::from("https://z1.m1907.cn/api/v?");
    url = url + "z=" + &finalMd5;
    url = url + "&jx=" + "刺杀小说家";
    url = url + "&s1ig=" + &slig.to_string();
    println!("url:{}", url);
}