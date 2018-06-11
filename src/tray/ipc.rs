use std::thread;
use std::time::Duration;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;
use std::sync::mpsc;
use chan;
use chan::{Sender, Receiver};

// TODO: use less threads

const SOCKET_PATH_SRV: &str = "/tmp/cakeytray-ipc-srv";
const SOCKET_PATH_RCV: &str = "/tmp/cakeytray-ipc-rcv";

pub fn get_server() -> (Sender<String>, mpsc::Receiver<String>){
    // remove files from last time
    remove_file(SOCKET_PATH_SRV);
    remove_file(SOCKET_PATH_RCV);

    let (tx_snd, rx_snd): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let (tx_rcv, rx_rcv): (Sender<String>, Receiver<String>) = chan::sync(0);

    thread::spawn(move || {
        let listener = UnixListener::bind(SOCKET_PATH_SRV).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // receive data
                    thread::spawn(move || {
                        let mut current = [0; 1];
                        let mut msg: Vec<u8> = Vec::new();
                        loop {
                            thread::sleep(Duration::new(0, 10000000));
                            if let Ok(_) = stream.read(&mut current) {
                                if current[0] == 0xA {
                                    let mut command = String::from_utf8(msg.clone()).unwrap();
                                    command.push_str("\n");
                                    msg.clear();
                                    println!("server {:#?}", command);
                                    tx_snd.send(command.trim().to_string());
                                } else {
                                    msg.push(current[0]);
                                }
                            }
                        }
                    });
                    // send data
                    let mut conn = UnixStream::connect(SOCKET_PATH_RCV).unwrap();
                    loop {
                        thread::sleep(Duration::new(0, 10000000));
                        if let Some(mut data) = rx_rcv.recv() {
                            data.push_str("\n");
                            conn.write(&data.into_bytes());
                        }
                    }
                },
                Err(err) => {
                    /* connection failed */
                    println!("err {:#?}", err);
                    break;
                }
            }
        }
    });

    (tx_rcv, rx_snd)
}

pub fn get_client() -> (Sender<String>, Receiver<String>) {

    let (tx_snd, rx_snd): (Sender<String>, Receiver<String>) = chan::sync(0);
    let (tx_rcv, rx_rcv): (Sender<String>, Receiver<String>) = chan::sync(0);

    thread::spawn(move || {
        let mut listener = UnixListener::bind(SOCKET_PATH_RCV).unwrap();
        let mut conn = UnixStream::connect(SOCKET_PATH_SRV).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // receive data
                    thread::spawn(move || {
                        let mut current = [0; 1];
                        let mut msg: Vec<u8> = Vec::new();
                        loop {
                            thread::sleep(Duration::new(0, 10000000));
                            if let Ok(_) = stream.read(&mut current) {
                                if current[0] == 0xA {
                                    let mut command = String::from_utf8(msg.clone()).unwrap();
                                    command.push_str("\n");
                                    msg.clear();
                                    println!("client {:#?}", command);
                                    tx_snd.send(command.trim().to_string());
                                } else {
                                    msg.push(current[0]);
                                }
                            }
                        }
                    });
                    // send data
                    loop {
                        thread::sleep(Duration::new(0, 10000000));
                        if let Some(mut data) = rx_rcv.recv() {
                            data.push_str("\n");
                            conn.write(&data.into_bytes());
                        }
                    }
                },
                Err(err) => {
                    /* connection failed */
                    println!("err {:#?}", err);
                    break;
                },
            }
        }
    });

    (tx_rcv, rx_snd)
}
