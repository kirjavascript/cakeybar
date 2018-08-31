use wm::ipc::parser::{Command, Selectors, Selector};
use std::fmt;

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            self.0.iter().map(|selector| match selector {
                Selector::Id(name) => format!("#{}", name),
                Selector::Class(name) => format!(".{}", name),
            }).collect::<Vec<String>>().join(", ")
        })
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::ReloadConfig(None) => {
                write!(f, "reloading config")
            },
            Command::ReloadConfig(Some(path)) => {
                write!(f, "reloading config {}", path)
            },
            Command::ReloadTheme(None) => {
                write!(f, "reloading theme")
            },
            Command::ReloadTheme(Some(path)) => {
                write!(f, "reloading theme {}", path)
            },
            Command::Show(selectors) => {
                write!(f, "showing {}", selectors)
            },
            Command::Hide(selectors) => {
                write!(f, "hiding {}", selectors)
            },
            _ => write!(f, "TODO: help"),
        }
    }
}
