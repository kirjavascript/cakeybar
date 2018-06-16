extern crate clap;
use clap::{Arg, App};
use std::process::Command;
use std::env::current_exe;
// ipc
extern crate ipc;
use ipc::unix_sock_stream::*;
use ipc::{Sender, Receiver};
// extern crate futures;
// use futures::future::Future;
// use std::io::Error;

const IPC_PATH: &str = "cakeytray-ipc";

type Data = String;

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

    let tx = UnixSockStreamServer::<i32>::new(IPC_PATH).unwrap();

    subprocess();

    loop {
        std::thread::sleep(std::time::Duration::new(0, 10000000));
            tx.send(9).unwrap();
        if let Ok(val) = tx.recv() {
            println!("first {:#?}", val);
        }
        else {
            eprintln!("{:#?}", "err");
        }
    }
}

fn second_main() {
    println!("{:#?}", "second process");
    let rx = UnixSockStreamClient::<i32>::new(IPC_PATH).unwrap();
    rx.send(5).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::new(0, 10000000));
            rx.send(1).unwrap();
        if let Ok(val) = rx.recv() {
            println!("second {:#?}", val);
        }
        else {
            eprintln!("{:#?}", "sec err");
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
