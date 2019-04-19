use crate::wm::ipc::commands::*;
use crate::wm::events::Event;
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
            wm_util.display_windows(&selectors, true);
        },
        Command::Hide(selectors) => {
            wm_util.display_windows(&selectors, false);
        },
        Command::Focus(selector) => {
            wm_util.emit(Event::Focus(selector.get_name()));
        },
        _ => {},
    }
}
