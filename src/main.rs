extern crate clap;

use clap::{Arg, App, SubCommand};

//use std::io::{ stdout, BufWriter };
//use std::env;
//use std::fs;
//use std::fmt;

fn main() {
    let matches = App::new("myapp")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .args_from_usage(
                              "-c, --config=[FILE] 'Sets a custom config file'
                              <INPUT>              'Sets the input file to use'
                              -v...                'Sets the level of verbosity'")
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches();
    
    
//    let out = b"Hello fellow Rustaceans!";
//    let width = 24;
//
//    let mut writer = BufWriter::new(stdout());
//    say(out, width, &mut writer).unwrap();
//    
//    println!("Hello, world!");
    
       // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
//    println!("{} days", 31);

//    // Without a suffix, 31 becomes an i32. You can change what type 31 is
//    // by providing a suffix. The number 31i64 for example has the type i64.
//
//    // There are various optional patterns this works with. Positional
//    // arguments can be used.
//    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//
//    // As can named arguments.
//    println!("{subject} {verb} {object}",
//             object="the lazy dog",
//             subject="the quick brown fox",
//             verb="jumps over");
//
//    // Special formatting can be specified after a `:`.
//    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
//
//    // You can right-align text with a specified width. This will output
//    // "     1". 5 white spaces and a "1".
//    println!("{number:>width$}", number=1, width=6);
//
//    // You can pad numbers with extra zeroes. This will output "000001".
//    println!("{number:0>width$}", number=1, width=6);
//
//    // Rust even checks to make sure the correct number of arguments are
//    // used.
//    println!("My name is {0}, {1} {0}", "Bond");
//    // FIXME ^ Add the missing argument: "James"
//
//    // Create a structure named `Structure` which contains an `i32`.
//    #[allow(dead_code)]
//    struct Structure(i32);
//
//    // However, custom types such as this structure require more complicated
//    // handling. This will not work.
//    println!("This struct `{}` won't print...", Structure(3));
//    // FIXME ^ Comment out this line.
//    
//    // --snip--
//    let args: Vec<String> = env::args().collect();
//    let query = &args[1];
//    let filename = &args[2];
//
//    println!("Searching for {}", query);
//    println!("In file {}", filename);
//
//    let contents = fs::read_to_string(filename)
//        .expect("Something went wrong reading the file");
//
//    println!("With text:\n{}", contents);

}
