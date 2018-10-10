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

fn main(){
    parse_command();
    println!("Hello, world!");
}
