mod ansi;
mod len;

//use len::len;
//use ansi::ansi;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    println!("{}", args[1]);
    let subcommand = &*args[1];

    match subcommand {
        "len" => len::main(args),
        "view" => println!("Run view\n"),
        "ansi" => ansi::main(args),
        _ => println!("Print out list of commands"),
    }
}
