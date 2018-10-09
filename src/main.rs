#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use std::io;
use clap::{Arg};

fn parse_command(){
    let app = app_from_crate!()
    .arg(Arg::from_usage("<TEXT> 'ファイルを指定してください.'"));
    let matches = app.get_matches();

    if let Some(o) = matches.value_of("TEXT") {
        println!("Value for TEXT: {}", o);
    }
}

fn main(){
    parse_command();
    println!("Hello, world!");
}
