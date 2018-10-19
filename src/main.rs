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

fn main(){
    // parse_command();
    // let n = getint();

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (k, v) in &score {
        println!("{} was born in {}", k, v);
    }
    let team_name = String::from("Blue");
    // 自分でDisplayメソッドを作るしかない
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    // println!("{}", &sc); 
    cli_tool::client::connect();
}
