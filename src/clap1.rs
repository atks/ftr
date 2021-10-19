extern crate clap;
use clap::{Arg, App, SubCommand};

pub fn main(args: Vec<String>) {
    
    println!("in calp1");
    
    // basic app information
    let app = App::new("hello-clap")
        .version("1.0")
        .about("Says hello")
        .author("Michael Snoyman");

    // Define the name command line option
    let name_option = Arg::with_name("name")
        .long("name") // allow --name
        .takes_value(true)
        .help("Who to say hello to")
        .required(true);

    // now add in the argument we want to parse
    let app = app.arg(name_option);

    let app = app.subcommand(SubCommand::with_name("clap")
                .about("tests things")
                .arg(Arg::with_name("case")
                .long("case")
                .takes_value(true)
                .help("the case to test")));

    // extract the matches
    let matches = app.get_matches_from(args);

    // Extract the actual name
    let name = matches.value_of("name")
        .expect("This can't be None, we said it was required");

    println!("Hello, {}!", name);
}