extern crate needletail;
use needletail::{parse_fastx_file};
extern crate clap;
use clap::{ArgMatches};


//print out length of sequences
//ftr len a.fastq.gz
pub fn len(args: &ArgMatches) {

    println!("running len")
    // let filename = &args[2];
    // let mut reader = parse_fastx_file(&filename).expect("invalid record");

    // while let Some(record) = reader.next() {
    //     let seqrec = record.expect("invalid record");
    //     println!("{}", seqrec.num_bases());
    // }
}
