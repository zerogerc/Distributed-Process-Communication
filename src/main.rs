#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate process_communication;

use process_communication::server::ProcessServer;
use process_communication::client::ProcessClient;
use process_communication::utils::*;
use process_communication::process::*;
use std::env;
use std::sync::RwLock;
use std::io;
use regex::Regex;

lazy_static! {
    static ref CONSOLE_DELIMETERS: Regex = Regex::new(r"\s+|\s*:\s*").unwrap();
}

fn parse_and_send(process: &Process, input: String) {
    let parts: Vec<&str> = CONSOLE_DELIMETERS.split(input.trim()).collect();

    if parts[0].eq("send") && parts[1].eq("to") && parts[3].eq("msg") {
        let id: u32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid request");
                return
            },
        };

        process.sendMessage(id, String::from(parts[4]));
    } else {
        println!("Invalid request");
    }
}

fn main() {
    let mut id: u32 = 0;
    for (num, argument) in env::args().enumerate() {
        if num == 0 { 
            continue; 
        }
        
        id = match argument.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("First argument must be a valid port");
                break;
            }
        }
    }

    let process = Process::new(id);
    let listener = process.startProcess();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().is_empty() {
            println!("Server is stopped");
            break;
        }

        parse_and_send(&process, input);
    }
}