use std::fmt;
use crate::wm::ipc::commands::*;

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            self.0
                .iter()
                .map(|selector| selector.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        })
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Selector::Id(name) => format!("#{}", name),
            Selector::Class(name) => format!(".{}", name),
        })
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::ReloadConfig(None) => write!(f, "reloading config"),
            Command::ReloadConfig(Some(path)) => write!(f, "reloading config {}", path),
            Command::ReloadTheme(None) => write!(f, "reloading theme"),
            Command::ReloadTheme(Some(path)) => write!(f, "reloading theme {}", path),
            Command::Show(selectors) => write!(f, "showing {}", selectors),
            Command::Focus(selector) => write!(f, "focus {}", selector),
            Command::Hide(selectors) => write!(f, "hiding {}", selectors),
            Command::Help(topic) => write!(f, "w:TODO: show help for {:?}", topic),
        }
    }
}
