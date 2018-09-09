mod listen;

pub use self::listen::listen;

use wm;
use wm::workspace::Workspace;

use std::env;
use std::io::{Error, Read, Write};
use std::os::unix::net::UnixStream;

pub fn connect() -> Result<UnixStream, Error> {
    let stream_file = env::var("BSPWM_SOCKET").unwrap_or("/tmp/bspwm_0_0-socket".to_string());

    UnixStream::connect(stream_file)
}

//https://github.com/baskerville/bspwm/blob/336095739e2de94109e55e544c806770316c822c/doc/bspwm.1.asciidoc

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

pub fn cycle_workspace(forward: bool, monitor_index: i32) {
    if let Ok(mut stream) = connect() {
        let workspaces = get_workspaces(&mut stream);

        let next_opt = wm::workspace::get_next(&workspaces, forward, monitor_index);

        if let Some(next) = next_opt {
            let command = format!("desktop -f {}", next.name);
            // TODO: reuse the connection
            run_command(command).ok();
        }
    }
}

// WMeDP1:oI:OII:fIII:fIV:fV:fVI:fVII:fVIII:fIX:fX:LT:TT:G
// WMDVI-I-1:OI:fII:oIII:fIV:fV:fVI:fVII:fVIII:fIX:fX:LT:TT:G:mDVI-D-0:ODesktop:LT:TT:G s
pub fn parse_workspaces(string: String) -> Vec<Workspace> {
    let mut workspaces: Vec<Workspace> = Vec::new();

    let string = &string[1..].trim();
    let indices: Vec<(usize, &str)> = string.match_indices(&['m', 'M'][..]).collect();
    let monitors = indices
        .iter()
        .enumerate()
        .map(|(i, (start, status))| {
            let end_opt = indices.get(i + 1).map(|x| x.0);
            let slice = if let Some(end) = end_opt {
                &string[start + 1..end]
            } else {
                &string[start + 1..]
            };
            (*status, slice)
        })
        .collect::<Vec<(&str, &str)>>();

    for monitor in monitors {
        let mut text: String = monitor.1.to_string();
        let mut tokens: Vec<String> = Vec::new();
        let mon_is_active: bool = monitor.0 == "M";
        while let Some(loc) = text.find(":") {
            let mut text_clone = text.clone();
            let (head, tail) = text_clone.split_at_mut(loc + 1);
            text = tail.to_string();
            tokens.push(head.trim_matches(':').to_string());
        }

        if let Some(name) = tokens.get(0) {
            let output = &name[..];
            &tokens[1..].iter().enumerate().for_each(|(i, token)| {
                let mut token_clone = token.clone();
                let (head, tail) = token_clone.split_at_mut(1);
                if head == "u" || head == "U" || head == "o" || head == "O" || head == "F" {
                    let uppercase = head.chars().next().unwrap().is_uppercase();
                    workspaces.push(Workspace {
                        number: i as i32 + 1,
                        name: tail.to_string(),
                        visible: uppercase,
                        focused: uppercase && mon_is_active,
                        urgent: head == "u" || head == "U",
                        output: output.to_string(),
                    });
                }
            });
        }
    }

    workspaces
}
