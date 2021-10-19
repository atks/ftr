extern crate needletail;
use needletail::{parse_fastx_file, Sequence};

pub fn main(args: Vec<String>) {
    let filename = &args[2];
    let mut reader = parse_fastx_file(&filename).expect("invalid record");

    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        println!("{}", seqrec.num_bases());
    }
}
