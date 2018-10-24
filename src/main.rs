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

struct Data{
    dep: Vec<String>,
    name: Vec<String>,
}

fn main(){
    // parse_command();
    // let n = getint();
    type Map = HashMap<String, Vec<String>>;
    let mut map = Map::new();
    let depart = vec!["Engineering".to_string(), "Sales".to_string(), "HRD".to_string()];
    
    let mut data = Data{
        dep: vec!["Engineering".to_string(), "Sales".to_string(), "HRD".to_string()],
        name: vec!["Alice".to_string(), "Bob".to_string(), "Mike".to_string(), "Ant".to_string()],
    };
    for k in &depart{
        map.insert(k.to_string(), Vec::new());
    }
    for d in data.dep.iter(){
        for v in data.name.iter(){
            map.get_mut(&d.to_string()).unwrap().push(v.to_string());
        }
    }
    for (k, v) in &mut map{
        println!("type: {}", k);
        v.sort();
        for per in v{
            println!("{}", per);
        }
    }
    cli_tool::client::connect();
}
