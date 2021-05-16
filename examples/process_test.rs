extern crate core;
extern crate local_encoding;
extern crate regex;

use std::process::{Command, Stdio, Child};
use std::error::Error as EType;
use std::io::{BufReader, Read, BufRead};
use std::thread;
use core::time;

use local_encoding::{Encoding, Encoder};
use regex::Regex;

fn main() {
    let url = " https://www.youtube.com/watch?v=-SsllwKLYJc";

    let mut cmd = Command::new("E:/workspace/rust-yt-dl/resources/youtube-dl.exe");
    //cmd.current_dir("E:/workspace/rust-yt-dl/resources/");
    cmd.args(&["--proxy", "127.0.0.1:1080"]);
    cmd.arg("--get-filename");
    let mut child = cmd
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut process = child.unwrap();

    let mut stdout_buffer = BufReader::new(process.stdout.take().unwrap());
    let mut stderr_buffer = BufReader::new(process.stderr.take().unwrap());

    //let mut stdout: String = String::new();
    //stdout_buffer.read_to_string(&mut stdout);
    let mut stderr: String = String::new();
    stderr_buffer.read_to_string(&mut stderr);

    let mut buf = Vec::<u8>::new();
    stdout_buffer.read_to_end(&mut buf).expect("read_until failed");


    process.wait();

    println!("get_file_name: {:?}", Encoding::OEM.to_string(buf.as_slice()).unwrap().trim_end());
    println!("error: {:?}", stderr);


    /*let capture = REGEX_NAME.captures(&stdout.trim());
    if stderr.is_empty() && capture.is_some() {
        let caps = capture.unwrap();
        println!("get_file_name: {:?}", stdout);
    } else {
        println!("error: {:?}", stderr);
    }*/


    //-------------------

    let output = Command::new("E:/workspace/rust-yt-dl/resources/youtube-dl.exe")
        .args(&["--proxy", "127.0.0.1:1080"])
        .arg("--get-filename")
        .arg(url)
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }



}