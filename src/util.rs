use std::{thread, env};
use libc;
use libc::{c_char};
use std::ffi::{CString, CStr};

pub fn get_config_dir() -> String {
    if let Ok(xdg_path) = env::var("XDG_CONFIG_HOME") {
        format!("{}/{}", xdg_path, ::NAME)
    } else if let Ok(home_path) = env::var("HOME") {
        format!("{}/.config/{}", home_path, ::NAME)
    } else {
        format!("~/.config/{}", ::NAME)
    }
}

pub fn run_command(command: String) {
    thread::spawn(clone!(command move || {
        let command = CString::new(command).unwrap();
        match CString::new(command) {
            Ok(command) => {
                let mode = b"r\0";
                let command = command.as_bytes_with_nul();
                let cmd_ptr: *const c_char = command.as_ptr() as _;
                let mode_ptr: *const c_char = mode.as_ptr() as _;
                unsafe {
                    let stream = libc::popen(cmd_ptr, mode_ptr);
                    if !stream.is_null() {
                        let stdout = CString::new("").unwrap();
                        let stdout_ptr: *mut c_char = stdout.as_bytes_with_nul().as_ptr() as _;
                        while !libc::fgets(stdout_ptr, 128, stream).is_null() {
                            if let Ok(stdout) = CStr::from_ptr(stdout_ptr).to_str() {
                                info!("{}", stdout.trim());
                            } else {
                                error!("reading command output");
                            }
                        }
                    }
                    libc::pclose(stream);
                }
            },
            Err(err) => {
                error!("{:?}", err);
            },
        }
    }));
}
