use crate::wm::ipc::commands::*;
use crate::wm::WMUtil;

pub fn run_command(wm_util: &WMUtil, cmd: Command) {
    match cmd {
        Command::ReloadTheme(path_opt) => {
            wm_util.load_theme(path_opt);
        },
        Command::ReloadConfig(path_opt) => {
            wm_util.reload_config(path_opt);
        },
        Command::Show(selectors) => {
            wm_util.display_bars(&selectors, true);
        },
        Command::Hide(selectors) => {
            wm_util.display_bars(&selectors, false);
        },
        _ => {},
    }
}
