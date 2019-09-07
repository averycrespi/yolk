#[macro_use]
extern crate clap;

use clap::{App, Arg};

use yolk::{YolkProgram, YololProgram};

use std::convert::TryInto;
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
        let yolk: YolkProgram = source.parse().unwrap_or_else(|e| panic!("{}", e));
        if debug {
            eprintln!("{:?}\n", yolk);
        }
        let yolol: YololProgram = yolk.try_into().unwrap_or_else(|e| panic!("{}", e));
        if debug {
            eprintln!("{:?}\n", yolol);
        }
        let optimized = yolol.optimize();
        if debug {
            eprintln!("{:?}\n", optimized);
        }
        println!("{}", optimized.to_string());
    }
}
