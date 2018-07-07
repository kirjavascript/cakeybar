//https://github.com/baskerville/bspwm/blob/336095739e2de94109e55e544c806770316c822c/doc/bspwm.1.asciidoc

use std::env;
use std::os::unix::net::{UnixStream};
use std::io::{Error, Write, Read};

pub fn connect() -> Result<UnixStream, Error> {
    let stream_file = env::var("BSPWM_SOCKET").unwrap_or("/tmp/bspwm_0_0-socket".to_string());

    UnixStream::connect(stream_file)
}

pub fn run_command(string: String) -> Result<String, Error> {
    let mut stream = connect()?;
    let mut cmd = string.replace(" ", "\0");
    cmd.push_str("\0");

    stream.write(cmd.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    Ok(response)
}

pub fn set_padding(is_top: bool, padding: i32) {
    let position = if is_top { "top" } else { "bottom" };

    run_command(format!("config {}_padding {}", position, padding)).ok();
}
