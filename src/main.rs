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


fn main(){
    // parse_command();
    // let n = getint();
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for i in &mut v{
        *i += 50;
    }
    for i in &v{
        println!("{}", i);
    }
    cli_tool::client::connect();
}
