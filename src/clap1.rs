

pub fn main(args: Vec<String>) {
    
    println!("in calp1");
    
    // basic app information
    let app = App::new("hello-clap")
        .version("1.0")
        .about("Says hello")
        .author("Michael Snoyman");

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