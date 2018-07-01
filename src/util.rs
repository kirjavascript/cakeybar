use std::{thread, str, env};
use std::process::Command;

pub fn get_config_dir() -> String {
    if let Ok(xdg_path) = env::var("XDG_CONFIG_HOME") {
        format!("{}/{}", xdg_path, ::NAME)
    } else if let Ok(home_path) = env::var("HOME") {
        format!("{}/.config/{}", home_path, ::NAME)
    } else {
        format!("~/.config/{}", ::NAME)
    }
}

pub fn run_command(exec: String) {
    thread::spawn(clone!(exec move || {
        let split: Vec<&str> = exec.split(" ").collect();
        let output = Command::new(split.get(0).unwrap_or(&""))
            .args(&split[1..])
            .output();
        match output {
            Ok(out) => {
                let stderr = str::from_utf8(&out.stderr).unwrap_or("");
                if stderr != "" {
                    warn!("{}", stderr);
                }
            },
            Err(err) => {
                warn!("{}: {}", err, exec);
            },
        }
    }));
}
