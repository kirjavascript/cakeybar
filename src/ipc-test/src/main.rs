extern crate clap;
use clap::{Arg, App};
use std::process::Command;
use std::env::current_exe;
// ipc
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;

extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use bincode::{serialize, deserialize};

const SOCKET_PATH_SRV: &str = "/tmp/cakeytray-ipc-srv";
const SOCKET_PATH_RCV: &str = "/tmp/cakeytray-ipc-rcv";

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Width(u32),
    Move(u32, u32),
}

fn main() {
    let matches = App::new("poop")
        .arg(Arg::with_name("alt")
             .short("a")
             .long("alt")
             .takes_value(false))
        .get_matches();

    if matches.is_present("alt") {
        // let path = matches.value_of("alt").unwrap();
        second_main();
        return ();
    }
    // main

    println!("{:#?}", "first process");

    let bg_hex = 0x221122 as u32;
    let icon_size = 20;

    // remove files from last time
    remove_file(SOCKET_PATH_SRV);
    remove_file(SOCKET_PATH_RCV);

    subprocess();

    let listener = UnixListener::bind(SOCKET_PATH_SRV).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();

                let mut conn = UnixStream::connect(SOCKET_PATH_RCV).unwrap();
                loop {
                    if let Ok(_) = stream.read(&mut current) {
                        msg.push(current[0]);

                        let msg_rcv_opt: Result<Message, _> = deserialize(&msg);
                        if let Ok(msg_rcv) = msg_rcv_opt {
                            println!("{:#?}", msg_rcv);
                            msg.clear();
                        }

                        // if current[0] == 0xA {
                        //     let command = String::from_utf8(msg.clone()).unwrap();
                        //     msg.clear();
                        //     println!("first {:#?}", command);
                    // conn.write(b"wow\n");
                        // } else {
                        // }
                    }
                }
            }
            Err(err) => {
                /* connection failed */
                println!("err {:#?}", err);
                break;
            }
        }
    }

}

fn second_main() {
    println!("{:#?}", "second process");

    let mut listener = UnixListener::bind(SOCKET_PATH_RCV).unwrap();
    let mut conn = UnixStream::connect(SOCKET_PATH_SRV).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // thread::spawn(move || {
                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();
                loop {
                    conn.write(&serialize(&Message::Width(1337)).unwrap());
                    conn.write(&serialize(&Message::Move(12, 34)).unwrap());
                    conn.write(&serialize(&Message::Move(12, 34)).unwrap());
                    conn.write(&serialize(&Message::Move(12, 0)).unwrap());
                    // if let Ok(_) = stream.read(&mut current) {
                    //     if current[0] == 0xA {
                    //         let command = String::from_utf8(msg.clone()).unwrap();
                    //         msg.clear();
                    //         println!("second {:#?}", command);
                    //         conn.write(b"wow3\n");
                    //     } else {
                    //         msg.push(current[0]);
                    //     }
                    // }
                }
            }
            Err(err) => {
                /* connection failed */
                println!("err {:#?}", err);
                break;
            }
        }
        }
}


fn subprocess() {
    if let Ok(path) = current_exe() {
        Command::new(path)
            .arg("-a")
            .spawn()
            .expect("failed");
    }
}
