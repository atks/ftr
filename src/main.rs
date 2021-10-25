extern crate clap;
use clap::{Arg, App, SubCommand};

//mod ansi;
mod len;
//mod view;

fn main() {

    let matches = App::new("ft")
                          .version("0.1.0")
                          .author("cavs")
                          .about("Tools related to dealing with sequences stored in FASTA/Q")
                          .subcommand(SubCommand::with_name("len")
                                      .about("gets length of sequences")
                                      .version("0.1.0")
                                      .author("adrian")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("len") {
        // "$ myapp test" was run
        println!("calculate length...");
    }

//    match matches.subcommand() {
//        "len" => println!("Running len"),
//        "view" => println!("Running view"),
//        "ansi" => println!("Running ansi"),
//        _ => println!("Print out list of commands"),
//    }
}
