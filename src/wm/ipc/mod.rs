mod listen;
pub use self::listen::listen;

use std::env;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

const DEFAULT_SOCKET: &str = "/tmp/cakeybar";

pub fn send(input: &str) {
    let mut conn = UnixStream::connect(get_socket_path()).unwrap();
    let send_res = conn.write(input.as_bytes());
    // if let Err(err) = send_res {
    //     warn!("{}", err);
    // }
}

pub fn get_socket_path() -> String {
    if let Ok(env) = env::var("CAKEYBAR_SOCKET") {
        format!("/tmp/{}", env)
    } else {
        DEFAULT_SOCKET.to_string()
    }
}
