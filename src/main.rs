extern crate cli_tool; // 外部クレートはsrcに構成する
extern crate failure;
#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::collections::HashMap;
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

struct Guess{
    val: i32,
}

impl Guess{
    fn new(val: i32) -> Guess{
        if val < 1 || val > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", val);
        }
        Guess{
            val
        }
    }
    fn val(&self) -> i32{
        self.val
    }
}

fn main(){
    // parse_command();
    let n = getint();
    let t = Guess::new(n);
    println!("{}", t.val);
    // ?演算子はResult型を返り値にする関数にしか使えない
    cli_tool::client::connect();
}
