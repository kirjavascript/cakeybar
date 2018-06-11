extern crate clap;
use clap::{Arg, App};
use std::process::Command;
use std::env::current_exe;
// ipc
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;

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

    subprocess();

    let listener = TcpListener::bind("127.0.0.1:21337").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // println!("{:#?}", stream);
                // let response = b"HELO";
                // stream.write(response).expect("Response failed");

                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();
                loop {
                    if let Ok(_) = stream.read(&mut current) {
                        if current[0] == 0xA {
                            let command = String::from_utf8(msg.clone()).unwrap();
                            msg.clear();
                            println!("first {:#?}", command);
                            stream.write(b"ACK\n");
                        } else {
                            msg.push(current[0]);
                        }

                    } else {
                        eprintln!("stream died");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }

}

fn second_main() {
    println!("{:#?}", "second process");

    let mut stream = TcpStream::connect("127.0.0.1:21337").unwrap();

    let mut current = [0; 1];
    let mut msg: Vec<u8> = Vec::new();
    loop {
        std::thread::sleep(std::time::Duration::new(0, 1000000000));
        // ignore the Result
        // let mut buf = [0; 4];
        // let b = stream.read(&mut buf); // ignore here too
        // buf.iter().for_each(|e| {
        //     println!("{}", e);
        // });
        let a = stream.write(b"HEL\n");

        if let Ok(_) = stream.read(&mut current) {
            if current[0] == 0xA {
                let command = String::from_utf8(msg.clone()).unwrap();
                msg.clear();
                println!("second {:#?}", command);
                stream.write(&msg.clone());
            } else {
                msg.push(current[0]);
            }

        } else {
            eprintln!("stream died");
            break;
        }
    }
        stream.shutdown(Shutdown::Both);
}


fn subprocess() {
    if let Ok(path) = current_exe() {
        Command::new(path)
            .arg("-a")
            .spawn()
            .expect("failed");
    }
}
