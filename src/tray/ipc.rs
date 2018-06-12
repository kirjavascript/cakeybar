use std::thread;
use std::time::Duration;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;
use std::sync::mpsc;
use chan;
use chan::{Sender, Receiver};

use bincode::{serialize, deserialize};

// TODO: use less threads

const SOCKET_PATH_SRV: &str = "/tmp/cakeytray-ipc-srv";
const SOCKET_PATH_RCV: &str = "/tmp/cakeytray-ipc-rcv";
const TEN_MS: u32 = 10000000;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Width(u16),
    Move(u32, u32),
    BgColor(u32),
    IconSize(u16),
}

pub fn get_server() -> (Sender<Message>, mpsc::Receiver<Message>){
    // remove files from last time
    remove_file(SOCKET_PATH_SRV);
    remove_file(SOCKET_PATH_RCV);

    let (tx_snd, rx_snd): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let (tx_rcv, rx_rcv): (Sender<Message>, Receiver<Message>) = chan::sync(0);

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
                            thread::sleep(Duration::new(0, TEN_MS));
                            if let Ok(_) = stream.read(&mut current) {
                                msg.push(current[0]);

                                let msg_rcv_opt: Result<Message, _> = deserialize(&msg);
                                if let Ok(msg_rcv) = msg_rcv_opt {
                                    // println!("server {:#?}", msg_rcv);
                                    tx_snd.send(msg_rcv);
                                    msg.clear();
                                }
                            }
                        }
                    });
                    // send data
                    let mut conn = UnixStream::connect(SOCKET_PATH_RCV).unwrap();
                    loop {
                        thread::sleep(Duration::new(0, TEN_MS));
                        if let Some(data) = rx_rcv.recv() {
                            let bytes = serialize(&data).unwrap();
                            conn.write(&bytes);
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

pub fn get_client() -> (Sender<Message>, Receiver<Message>) {

    let (tx_snd, rx_snd): (Sender<Message>, Receiver<Message>) = chan::sync(0);
    let (tx_rcv, rx_rcv): (Sender<Message>, Receiver<Message>) = chan::sync(0);

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
                            thread::sleep(Duration::new(0, TEN_MS));
                            if let Ok(_) = stream.read(&mut current) {
                                msg.push(current[0]);

                                let msg_rcv_opt: Result<Message, _> = deserialize(&msg);
                                if let Ok(msg_rcv) = msg_rcv_opt {
                                    // println!("client {:#?}", msg_rcv);
                                    tx_snd.send(msg_rcv);
                                    msg.clear();
                                }
                            }
                        }
                    });
                    // send data
                    loop {
                        thread::sleep(Duration::new(0, TEN_MS));
                        if let Some(data) = rx_rcv.recv() {
                            let bytes = serialize(&data).unwrap();
                            conn.write(&bytes);
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
