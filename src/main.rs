mod ansi;
mod len;
mod view;

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
        "view" => view::main(args),
        "ansi" => ansi::main(args),
        _ => println!("Print out list of commands"),
    }
}
