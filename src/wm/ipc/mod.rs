pub mod commands;
mod display;
mod listen;
pub mod parser;
pub mod exec;
pub use self::listen::listen;

use std::io::{Error, Read, Write};
use std::os::unix::net::UnixStream;

pub fn send(input: &str) -> Result<String, Error> {
    let mut conn = UnixStream::connect(crate::config::CAKEYBAR_SOCKET.to_owned())?;
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
        Err(err) => error!("{}", err.to_string().to_lowercase()),
    }
}
