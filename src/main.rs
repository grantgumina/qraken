extern crate clap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Kraken")
        .version("0.1.0")
        .author("Grant Gumina")
        .about("Allows you to run the same command on multiple machines over ssh")
        .arg(Arg::with_name("IP_ADDRESS_LIST")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Comma separated list of IP addresses (x.x.x.x,y.y.y.y)"))
        .arg(Arg::with_name("COMMAND"))
            .required(true)
            .takes_value(true)
            index(2)
            .help(""))
        .get_matches();

    let argument = matches.value_of("FIRST_ARG").unwrap();
    println!("{}", argument);
}
