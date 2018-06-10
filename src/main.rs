extern crate clap;
extern crate ssh2;

use std::io::prelude::*;
use clap::{Arg, App};
use std::net::TcpStream;
use ssh2::Session;

// fn extract_arguments(matches: &ArgMatches) -> (Vec<&str>, String, String, String) {
//     let address_list: Vec<_> = matches.values_of("IP_ADDRESS_LIST").unwrap().collect();
//     let username = matches.value_of("USERNAME").unwrap().to_string();
//     let password = matches.value_of("PASSWORD").unwrap().to_string();
//     let command = matches.value_of("COMMAND").unwrap().to_string();

//     (address_list, username, password, command)
// }

fn create_session(address: &str, username: &str, password: &str) -> (TcpStream, Session) {
    let tcp = TcpStream::connect(address).unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
        
    match session.userauth_password(username, password) {
        Ok(_r) => {
            println!("Successfully Authenticated to {}", address);
            (tcp, session)
        },
        Err(error) => {
            panic!("Failed to Authenticate: {}", error)
        }
    }
}

fn run_command(session: Session, command: &str) -> String {
    let mut channel = session.channel_session().unwrap();
    let mut s = String::new();
    channel.exec(command).unwrap();
    channel.read_to_string(&mut s).unwrap();
    
    s
}

fn main() {

    let matches = App::new("Kraken")
        .version("0.1.0")
        .author("Grant Gumina")
        .about("Allows you to run the same command on multiple machines over ssh")
        .arg(Arg::with_name("IP_ADDRESS_LIST")
            .short("l")
            .required(true)
            .multiple(true)
            .takes_value(true)
            .help("List of IP addresses (eg. '-l x.x.x.x y.y.y.y')"))
        .arg(Arg::with_name("PASSWORD")
            .short("p")
            .required(true)
            .takes_value(true)
            .help("Password for nodes being accessed"))
        .arg(Arg::with_name("USERNAME")
            .short("u")
            .required(true)
            .takes_value(true)
            .help("Password for nodes being accessed"))
        .arg(Arg::with_name("COMMAND")
            .short("c")
            .required(true)
            .takes_value(true)
            .help("Command to be executed on all servers specified"))
        .get_matches();

    let address_list: Vec<_> = matches.values_of("IP_ADDRESS_LIST").unwrap().collect();
    let username = matches.value_of("USERNAME").unwrap().to_string();
    let password = matches.value_of("PASSWORD").unwrap().to_string();
    let command = matches.value_of("COMMAND").unwrap().to_string();

    // let mut connections = Vec::new();

    // Create connections to each supplied address
    for address in address_list {
        let (_stream, session) = create_session(&address, &username, &password);
        // connections.push(session);
        let result = run_command(session, &command);
        println!("\nCommand Output:\n===============");
        print!("{}", result);
    }

    // let command = matches.value_of("COMMAND").unwrap();
    // println!("{}", command);
}
