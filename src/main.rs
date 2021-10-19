mod ansi;
mod len;
mod view;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcommand = &*args[1];

    match subcommand {
        "len" => len::main(args),
        "view" => view::main(args),
        "ansi" => ansi::main(args),
        _ => println!("Print out list of commands"),
    }
}
