#[macro_use]
extern crate clap;

use clap::{App, Arg};

use yolk::ast::YololNode;
use yolk::{optimize, parse, transpile};

use std::fs;

fn main() {
    let matches = App::new("yolkc")
        .version(crate_version!())
        .author("Avery Crespi <averycrespi@gmail.com>")
        .about("Transpiler for the Yolk language")
        .arg(
            Arg::with_name("infile")
                .help("input file")
                .takes_value(true)
                .short("i")
                .long("infile"),
        )
        .arg(
            Arg::with_name("debug")
                .help("print debug messages")
                .long("debug"),
        )
        .get_matches();

    let debug = matches.is_present("debug");

    if let Some(infile) = matches.value_of("infile") {
        let source = fs::read_to_string(infile).expect("cannot read from file");
        let yolk = parse(&source).unwrap_or_else(|e| panic!("{}", e));
        if debug {
            eprintln!("{:?}\n", yolk);
        }
        let (yolol, context) = transpile(&yolk).unwrap_or_else(|e| panic!("{}", e));
        if debug {
            eprintln!("{:?}\n", yolol);
        }
        let optimized = optimize(&yolol, &context);
        if debug {
            eprintln!("{:?}\n", optimized);
        }
        println!("{}", YololNode::format_as_program(&optimized));
    }
}
