extern crate clap;
use clap::{App, SubCommand, ArgMatches};
extern crate needletail;
use needletail::{parse_fastx_file};


//make sub command interface
pub fn make_subcmd() -> App<'static, 'static> {
    SubCommand::with_name("len")
}

pub fn run(args: &ArgMatches) {




    println!("running len")
    // let filename = &args[2];
    // let mut reader = parse_fastx_file(&filename).expect("invalid record");

    // while let Some(record) = reader.next() {
    //     let seqrec = record.expect("invalid record");
    //     println!("{}", seqrec.num_bases());
    // }
}
