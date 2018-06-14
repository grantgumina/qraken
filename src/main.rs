extern crate clap;
extern crate ssh2;
extern crate indicatif;
extern crate chrono;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::thread::spawn;
use chrono::prelude::*;
use clap::{Arg, App};
use std::sync::Arc;
use std::net::TcpStream;
use ssh2::Session;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};

fn create_session(address: &str, username: &str, password: &str) -> Result<(TcpStream, Session), ssh2::Error>{
    let tcp = TcpStream::connect(address).unwrap();
    let mut session = Session::new().unwrap();
    
    session.handshake(&tcp).unwrap();
    session.userauth_password(username, password)?;

    Ok((tcp, session))
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
            .help("Username for nodes being accessed"))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .required(false)
            .takes_value(true)
            .default_value("xxNONExx")
            .help("File where command results are dumped"))
        .arg(Arg::with_name("COMMAND")
            .short("c")
            .required(true)
            .takes_value(true)
            .help("Command to be executed on all servers specified"));
    
    let matches = app.get_matches();
    let address_list: Vec<_> = matches.values_of("IP_ADDRESS_LIST").unwrap().collect();

    // Setup progress bars
    let mp = Arc::new(MultiProgress::new());
    let bar_style = ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").progress_chars("##-");

    let mut guards = vec![];

    for (_iteration, address) in address_list.iter().enumerate() {

        let u = matches.value_of("USERNAME").unwrap().to_string();
        let p = matches.value_of("PASSWORD").unwrap().to_string();
        let c = matches.value_of("COMMAND").unwrap().to_string();
        let of = matches.value_of("OUTPUT").unwrap().to_string();
        let a = address.to_string();

        let pb = mp.add(ProgressBar::new(3));
        pb.set_style(bar_style.clone());

        guards.push(spawn(move || {

            match create_session(&a, &u, &p) {
                Ok(result) => {
                    let session = result.1;
                    pb.set_message(&format!("{}: Session created", a));
                    pb.inc(1);
                    
                    let command_result = run_command(&session, &c).to_string();

                    if of != "xxNONExx" {
                        pb.set_message(&format!("{}: Writing to specified output file", a));

                        match OpenOptions::new().append(true).create(true).open(of) {
                            Ok(output_file) => {
                                let file_contents = &format!("{}\n{}:\n{}\n", Utc::now(), &a, command_result);

                                let mut writer = std::io::BufWriter::new(&output_file);
                                writer.write_all(file_contents.as_bytes()).unwrap();

                                pb.inc(2);
                            },
                            Err(e) => {
                                pb.set_message(&format!("{}: Command Executed - File write error {}", a, e))
                            }
                        }
                    } else {
                        pb.set_message(&format!("{}: Command Executed", a));
                        pb.inc(2);
                    }
                    
                },
                Err(error) => {
                    pb.set_message(&format!("{}: {}", a, error));
                }
            }

            pb.finish();
        }));
    }

    mp.join().unwrap();

    for g in guards {
        g.join().unwrap();
    }

}