use std::ops::Mul;
use chrono::{DateTime, Utc, Local};
use std::any::type_name;

pub fn main() {
    let a = 1.0 * 1 as f64;
    println!("{}", a);

    let now: DateTime<Local> = Local::now();

    println!("type:{}", type_of(now));

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%Y-%m-%d %H:%M:%S").to_string());
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
