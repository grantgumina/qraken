extern crate clap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Kraken")
        .version("0.1.0")
        .author("Grant Gumina")
        .about("Allows you to run the same command on multiple machines over ssh")
        .arg(Arg::with_name("FIRST_ARG")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("No clue right now"))
        .get_matches();

    let argument = matches.value_of("FIRST_ARG").unwrap();
    println!("{}", argument);
}
