use std::env;
use lazy_static::lazy_static;

fn get_xdg(path: &str, default: &str) -> String {
    if let Ok(xdg_path) = env::var(path) {
        format!("{}/{}", xdg_path, crate::NAME)
    } else {
        let home = env::var("HOME").expect("POSIX requires you to set $HOME");
        format!("{}/{}/{}", home, default, crate::NAME)
    }
}

lazy_static! {
    pub static ref NO_COLOR: bool = env::var("NO_COLOR").is_ok();
    pub static ref BSPWM_SOCKET: String = env::var("BSPWM_SOCKET")
        .unwrap_or_else(|_| "/tmp/bspwm_0_0-socket".to_string());
    pub static ref CAKEYBAR_SOCKET: String = env::var("CAKEYBAR_SOCKET")
        .unwrap_or_else(|_| "/tmp/cakeybar".to_string());
    pub static ref CONFIG_DIR: String = get_xdg("XDG_CONFIG_HOME", ".config");
    pub static ref CACHE_DIR: String = get_xdg("XDG_CACHE_HOME", ".cache");
}
