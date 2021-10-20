extern crate clap;
use clap::{Arg, App, SubCommand};

mod ansi;
mod len;
mod view;

use std::env;

arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        SHA1,
        SHA256,
        Argon2
    }
}

fn main() {

    let matches = App::new()
                 .subcommand(SubCommand::with_name("analyze")
                    .about("Analyses the data from file")
                    .arg(Arg::with_name("input-file")
                    .short("i")
                    .default_value("default.csv")
                    .value_name("FILE")))
                 .subcommand(SubCommand::with_name("verify")
                    .about("Verifies the data")
                    .arg(Arg::with_name("algorithm")
                    .short("a")
                    .possible_values(&Algorithm::variants())
                    .require(true)
                    .value_name("ALGORITHM")))
                 .get_matches();


    match matches.subcommand() {
        "len" => len::main(args),
        "view" => view::main(args),
        "ansi" => ansi::main(args),
        _ => println!("Print out list of commands"),
    }
}
