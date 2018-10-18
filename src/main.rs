extern crate cli_tool; // 外部クレートはsrcに構成する
extern crate failure;
#[macro_use]
extern crate clap;

use std::io::prelude::*;
// use std::env;
use std::fs::File;
use clap::{Arg};

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

// 関数内で文字列に変更がないい場合はスライスで指定する
fn append_world(str1: &str) -> &str {
  let mut result = String::with_capacity(str1.len() + 6);
  result.push_str(str1);
  result.push_str(" World");
  return result;
}

fn main(){
    // parse_command();
    // let n = getint();
    
    println!("{}", append_world("Hello"));
    cli_tool::client::connect();
}
