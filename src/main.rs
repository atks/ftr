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

    let mut subcmd = "";
    if let ("len", Some(rm)) = matches.subcommand() {
        len::len(rm);
    }
}

fn print_commands() {

    println!("len - compute length");
    println!("gb2fasta - convert genbank to fasta");
}
