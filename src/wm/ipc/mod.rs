mod listen;
pub use self::listen::listen;
use std::env;

const DEFAULT_SOCKET: &str = "/tmp/cakeybar";

// pub fn send_message

pub fn get_socket_path() -> String {
    if let Ok(env) = env::var("CAKEYBAR_SOCKET") {
        format!("/tmp/{}", env)
    } else {
        DEFAULT_SOCKET.to_string()
    }
}

pub fn send(input: &str) {
    error!("TODO {}", input);
}
