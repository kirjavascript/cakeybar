use std::thread;
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

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Width(u16),
    Move(u32, u32),
    BgColor(u32),
    IconSize(u16),
}

pub fn get_server() -> (Sender<Message>, mpsc::Receiver<Message>){
    // remove files from last time
    remove_file(SOCKET_PATH_SRV).ok();
    remove_file(SOCKET_PATH_RCV).ok();

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
                            // stream.read appears to block
                            if let Ok(_) = stream.read(&mut current) {
                                msg.push(current[0]);

                                let msg_rcv_opt: Result<Message, _> = deserialize(&msg);
                                if let Ok(msg_rcv) = msg_rcv_opt {
                                    // info!("server {:#?}", msg_rcv);
                                    tx_snd.send(msg_rcv).unwrap();
                                    msg.clear();
                                }
                            }
                        }
                    });
                    // send data
                    let mut conn = UnixStream::connect(SOCKET_PATH_RCV).unwrap();
                    loop {
                        // thread::sleep(Duration::from_millis(DELAY_MS));
                        if let Some(data) = rx_rcv.recv() {
                            let bytes = serialize(&data).unwrap();
                            let send_res = conn.write(&bytes);
                            if let Err(err) = send_res {
                                warn!("{}", err);
                            }
                        }
                    }
                },
                Err(err) => {
                    error!("{:#?}", err);
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
        let listener = UnixListener::bind(SOCKET_PATH_RCV).unwrap();
        let mut conn = UnixStream::connect(SOCKET_PATH_SRV).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // receive data
                    thread::spawn(move || {
                        let mut current = [0; 1];
                        let mut msg: Vec<u8> = Vec::new();
                        loop {
                            if let Ok(_) = stream.read(&mut current) {
                                msg.push(current[0]);

                                let msg_rcv_opt: Result<Message, _> = deserialize(&msg);
                                if let Ok(msg_rcv) = msg_rcv_opt {
                                    // info!("client {:#?}", msg_rcv);
                                    tx_snd.send(msg_rcv);
                                    msg.clear();
                                }
                            }
                        }
                    });
                    // send data
                    loop {
                        // thread::sleep(Duration::from_millis(DELAY_MS));
                        if let Some(data) = rx_rcv.recv() {
                            let bytes = serialize(&data).unwrap();
                            let send_res = conn.write(&bytes);
                            if let Err(err) = send_res {
                                warn!("{:?}", err);
                            }
                        }
                    }
                },
                Err(err) => {
                    error!("{:#?}", err);
                    break;
                },
            }
        }
    });

    (tx_rcv, rx_snd)
}
