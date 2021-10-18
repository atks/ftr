mod len;

use len::len;
use std::env;

pub fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    println!("{}", args[1]);
    let subcommand = &*args[1];

    match subcommand {
        "len" => len(args),
        "view"  => println!("Run view\n"),
        _       => println!("Print out list of commands"),
    }
}
