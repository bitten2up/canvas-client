use reqwest;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io::Read;

// tokio let's us use "async" on our main functionn
#[tokio::main]
async fn main() {
    let filepath = ".auth";
    let path = Path::new(&filepath);
    let mut file = File::open(&path).expect("no auth key");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents.clone()),
        Err(e) => Err(e),
    };
    trim_newline(&mut contents);
    let url = format!(
        "https://canvas.instructure.com/api/v1/courses/{path}",
        path = ""
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", contents))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
