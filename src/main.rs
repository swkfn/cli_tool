#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::env;
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

fn fib(n :i32) -> i32{
    if n == 0{
        0
    }else{
        let mut f1 = 0;
        let mut f2 = 1;
        let mut t = 0;
        for _ in 0..n-2{
            t = f1 + f2;
            f1 = f2;
            f2 = t;
        }
        f2
    }
}

fn main(){
    // parse_command();
    let n = getint();
    println!("{}", fib(n));
    println!("Hello, world!");
}
