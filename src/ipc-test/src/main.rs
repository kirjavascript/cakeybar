extern crate clap;
use clap::{Arg, App};
use std::process::Command;
use std::env::current_exe;
// ipc
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;

const SOCKET_PATH_SRV: &str = "/tmp/cakeytray-ipc/srv";
const SOCKET_PATH_RCV: &str = "/tmp/cakeytray-ipc/rcv";

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

    let listener = UnixListener::bind(SOCKET_PATH).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // thread::spawn(move || {
                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();
                let mut flag = false;
                loop {
                    flag = !flag;
                    if flag {
                        if let Ok(_) = stream.read(&mut current) {
                            if current[0] == 0xA {
                                let command = String::from_utf8(msg.clone()).unwrap();
                                msg.clear();
                                println!("first {:#?}", command);
                            } else {
                                msg.push(current[0]);
                            }
                        }
                    } else {
                        stream.write(b"ACK\n");
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

    // let listener = TcpListener::bind("127.0.0.1:21337").unwrap();

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             // println!("{:#?}", stream);
    //             // let response = b"HELO";
    //             // stream.write(response).expect("Response failed");

    //             let mut current = [0; 1];
    //             let mut msg: Vec<u8> = Vec::new();
    //             loop {
    //                 if let Ok(_) = stream.read(&mut current) {
    //                     if current[0] == 0xA {
    //                         let command = String::from_utf8(msg.clone()).unwrap();
    //                         msg.clear();
    //                         println!("first {:#?}", command);
    //                         stream.write(b"ACK\n");
    //                     } else {
    //                         msg.push(current[0]);
    //                     }

    //                 } else {
    //                     eprintln!("stream died");
    //                     break;
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             println!("Unable to connect: {}", e);
    //         }
    //     }
    // }

}

fn second_main() {
    println!("{:#?}", "second process");

    let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();
    // let mut response = String::new();
    // stream.read_to_string(&mut response).unwrap();
    // println!("{}", response);

                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();
                let mut flag = true;
    loop {
        // match stream.write(b"hello world\n") {
        //     Ok(_) => {
        //         // println!("{:#?}", "sent message");
        //     },
        //     Err(err) => {
        //         eprintln!("{:#?}", err);
        //     },
        // }

        flag = !flag;
        if flag {
            if let Ok(_) = stream.read(&mut current) {
                if current[0] == 0xA {
                    let command = String::from_utf8(msg.clone()).unwrap();
                    msg.clear();
                    println!("first {:#?}", command);
                } else {
                    msg.push(current[0]);
                }
            }
        } else {
            stream.write(b"ACK\n");
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
