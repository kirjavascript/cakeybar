mod listen;

pub use self::listen::listen;

use wm::workspace::Workspace;

use std::env;
use std::os::unix::net::{UnixStream};
use std::io::{Error, Write, Read};

pub fn connect() -> Result<UnixStream, Error> {
    let stream_file = env::var("BSPWM_SOCKET")
        .unwrap_or("/tmp/bspwm_0_0-socket".to_string());

    UnixStream::connect(stream_file)
}

// TODO: multimonitor

//https://github.com/baskerville/bspwm/blob/336095739e2de94109e55e544c806770316c822c/doc/bspwm.1.asciidoc
//
// bspc wm -D
// bspc -D any.local.focused

// util

pub fn write_message(stream: &mut UnixStream, string: String) -> Result<usize, Error> {
    let msg = format!("{}{}", string.replace(" ", "\0"), "\0");
    stream.write(msg.as_bytes())
}

pub fn query_message(stream: &mut UnixStream, string: String) -> Result<String, Error> {
    write_message(stream, string)?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

pub fn get_workspaces(stream: &mut UnixStream) -> Vec<Workspace> {
    if let Ok(response) = query_message(stream, "wm -g".to_string()) {
        parse_workspaces(response)
    } else {
        Vec::new()
    }
}

// connect and send

pub fn run_command(string: String) -> Result<String, Error> {
    let mut stream = connect()?;
    query_message(&mut stream, string)
}

pub fn set_padding(is_top: bool, padding: i32) {
    let position = if is_top { "top" } else { "bottom" };

    run_command(format!("config {}_padding {}", position, padding)).ok();
}

pub fn cycle_workspace(forward: bool) {
    let direction = if forward { "next" } else { "prev" };

    // TODO: get list of desktops and current
    if let Ok(mut stream) = connect() {

        // cycle the "current" monitor
        write_message(&mut stream, format!("desktop -f {}.local", direction)).ok();
    }

}

// WMeDP1:oI:OII:fIII:fIV:fV:fVI:fVII:fVIII:fIX:fX:LT:TT:G
pub fn parse_workspaces(string: String) -> Vec<Workspace> {
    let monitors = string.trim().split("\n").collect::<Vec<&str>>();
    let mut workspaces: Vec<Workspace> = Vec::new();

    for monitor in monitors {
        let mut text: String = monitor.to_string();
        let mut tokens: Vec<String> = Vec::new();
        while let Some(loc) = text.find(":") {
            let mut text_clone = text.clone();
            let (head, tail) = text_clone.split_at_mut(loc + 1);
            text = tail.to_string();
            tokens.push(head.trim_matches(':').to_string());
        }

        if let Some(name) = tokens.get(0) {
            let output = &name[2..]; // ignores 'WM'
            &tokens[1..].iter().enumerate().for_each(|(i, token)| {
                let mut token_clone = token.clone();
                let (head, tail) = token_clone.split_at_mut(1);
                if head == "u" || head == "U" || head == "o" || head == "O" || head == "F" {
                    let uppercase = head.chars().next().unwrap().is_uppercase();
                    workspaces.push(Workspace {
                        number: i as i32 + 1,
                        name: tail.to_string(),
                        // TODO: get which monitor is focused
                        visible: uppercase,
                        focused: uppercase,
                        urgent: head == "u" || head == "U",
                        output: output.to_string(),
                    });
                }
            });
        }
    }

    workspaces
}
