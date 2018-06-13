extern crate clap;
extern crate ssh2;
extern crate indicatif;

use std::io::prelude::*;
use std::thread::spawn;
use clap::{Arg, App};

use std::sync::Arc;
use std::net::TcpStream;
use ssh2::Session;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};

fn create_session(address: &str, username: &str, password: &str) -> (TcpStream, Session) {
    let tcp = TcpStream::connect(address).unwrap();
    
    // mutable reference will be copied...
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    
    match session.userauth_password(username, password) {
        Ok(_r) => {
            (tcp, session)
        },
        Err(error) => {
            panic!("Failed to Authenticate: {}", error)
        }
    }
}

fn run_command(session: &Session, command: &str) -> String {
    let mut channel = session.channel_session().unwrap();
    let mut s = String::new();
    
    channel.exec(command).unwrap();
    channel.read_to_string(&mut s).unwrap();

    s
}

fn main() {

    let app = App::new("Kraken")
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
            .help("Command to be executed on all servers specified"));
    
    let matches = app.get_matches();
    
    let address_list: Vec<_> = matches.values_of("IP_ADDRESS_LIST").unwrap().collect();
    let username = matches.value_of("USERNAME").unwrap().to_string();
    let password = matches.value_of("PASSWORD").unwrap().to_string();
    let command = matches.value_of("COMMAND").unwrap().to_string();

    let mp = Arc::new(MultiProgress::new());
    let bar_style = ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").progress_chars("##-");

    let mut guards = vec![];

    for (_iteration, address) in address_list.iter().enumerate() {
        let c = command.to_string();
        let u = username.to_string();
        let p = password.to_string();
        let a = address.to_string();

        let pb = mp.add(ProgressBar::new(3));
        pb.set_style(bar_style.clone());

        guards.push(spawn(move || {
            let (_stream, session) =  create_session(&a, &u, &p);
            pb.set_message(&format!("{}: Session created", a));
            pb.inc(1);
            let result = run_command(&session, &c);
            pb.set_message(&format!("{}: Command Executed", a));
            pb.inc(2);
            pb.finish();
        }));
    }

    mp.join().unwrap();

    for g in guards {
        g.join().unwrap();
    }

}