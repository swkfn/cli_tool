#![allow(dead_code)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use clap::{Arg};

struct User{
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// {:?} -> Debug出力, 構造体で使う場合は#[derive~]を宣言
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// classのメソッドとメンバーは別々的な感じ?
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn is_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn set_user(email: String, username: String) -> User{
    User{
        username,
        email,
        active: true,
        sign_in_count: 2,
    }
}

fn parse_command(){
    let app = app_from_crate!()
            .arg(Arg::from_usage("<TEXT> 'ファイルを指定してください.'"));
    let matches = app.get_matches();

    if let Some(filename) = matches.value_of("TEXT") {
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        println!("Value for TEXT:\n{}", contents);
    }
}

fn getint() -> i32{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let vec: Vec<&str> = line.split_whitespace().collect();
    vec[0].parse().unwrap_or(0)
}

fn main(){
    // parse_command();
    // let n = getint();
    //let user1 = set_user(String::from("swkfn@example.com"), String::from("swkfn"));
    // ..user1で同じ設定をコピーできる

    //let rect1 = Rectangle { width: 30, height: 50 };
    //println!("rec1 is {}", rect1.area());

    let some_u8_value = Some(3);
    println!("{:?}", some_u8_value);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
