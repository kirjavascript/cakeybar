pub mod commands;
mod display;
mod listen;
mod parser;
pub use self::listen::listen;

use std::env;
use std::io::{Error, Read, Write};
use std::os::unix::net::UnixStream;

const DEFAULT_SOCKET: &str = "/tmp/cakeybar";

pub fn send(input: &str) -> Result<String, Error> {
    let mut conn = UnixStream::connect(get_socket_path())?;
    conn.write(input.as_bytes())?;
    let mut data = String::new();
    conn.read_to_string(&mut data)?;
    Ok(data)
}

pub fn send_message(input: &str) {
    info!("sending {:?} via IPC...", input);
    match send(input) {
        Ok(res) => {
            if res.starts_with("e:") {
                error!("{}", &res[2..]);
            } else if res.starts_with("w:") {
                warn!("{}", &res[2..]);
            } else {
                info!("{}", res);
            }
        }
        Err(err) => error!("{}", err),
    }
}

pub fn get_socket_path() -> String {
    if let Ok(env) = env::var("CAKEYBAR_SOCKET") {
        format!("/tmp/{}", env)
    } else {
        DEFAULT_SOCKET.to_string()
    }
}
