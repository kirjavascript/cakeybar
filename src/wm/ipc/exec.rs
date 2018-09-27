use wm::ipc::commands::*;
use wm::WMUtil;

pub fn run_command(wm_util: &WMUtil, cmd: Command) {
    match cmd {
        Command::ReloadTheme(path_opt) => {
            wm_util.load_theme(path_opt);
        },
        Command::ReloadConfig(path_opt) => {
            wm_util.reload_config(path_opt);
        },
        Command::Show(selectors) => {
            let bar_names = wm_util.get_bar_names();
            let bars = get_bars_from_selectors(&selectors, bar_names);

            if selectors.len() == bars.len() {
                // if we only have bars
                wm_util.display_bars(bars, true);
            } else {
                // otherwise targets id/classes from specific bars
                wm_util.display_components(bars, selectors, true);
            }
        },
        Command::Hide(selectors) => {
            let bar_names = wm_util.get_bar_names();
            let bars = get_bars_from_selectors(&selectors, bar_names);

            if selectors.len() == bars.len() {
                wm_util.display_bars(bars, false);
            } else {
                wm_util.display_components(bars, selectors, false);
            }
        },
        _ => {},
    }
}


fn get_bars_from_selectors(selectors: &Selectors, bar_names: Vec<String>) -> Vec<String> {
    selectors.0.iter()
        .filter(|selector| {
            if let Selector::Id(name) = selector {
                bar_names.contains(&name)
            } else {
                false
            }
        })
        .map(|selector| selector.get_name())
        .collect::<Vec<String>>()
}
