extern crate clap;
use clap::{App, SubCommand};

mod len;

fn main() {

    let matches = App::new("ftr")
                          .version("0.1.0")
                          .author("cavs")
                          .about("Tools related to dealing with sequences stored in FASTA/Q")
                          .subcommand(len::make_subcmd())
                          .get_matches();

    if let ("len", Some(rm)) = matches.subcommand() {
        len::run(rm);
    }
    else {
        println!("len - compute length");
        println!("gb2fasta - convert genbank to fasta");
    }
}
