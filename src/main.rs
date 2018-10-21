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
    let n = getint();
    let mut v = Vec::new();
    let mut sum = 0;
    let mut map = HashMap::new();
    for i in 0..n{
        let a = getint();
        sum += a;
        let cnt = map.entry(a).or_insert(0);
        *cnt += 1;
        v.push(a);
    }
    v.sort();
    println!("sorted!");
    for i in &mut v{
        println!("{}", i);
    }
    println!("mean :{}", sum / (v.len() as i32));
    let mut max = -1;
    let mut idx = 0;
    for (key, val) in &mut map{
        if max < *val{
            max = *val;
            idx = *key;
        }
    }
    println!("mode :{}, count :{}", idx, max);
    cli_tool::client::connect();
}
