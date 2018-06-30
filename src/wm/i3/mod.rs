mod listen;

// reexported public interface
pub use self::listen::listen;

// utils
use i3ipc::{I3Connection, EstablishError};

pub fn connect() -> Result<I3Connection, EstablishError> {
    I3Connection::connect()
}

///

// mod gtk {
//     use gdk::{Screen, ScreenExt};
//     pub fn get_monitor_name(monitor_index: i32) -> (bool, String) {
//         let screen = Screen::get_default().unwrap();
//         let monitor_name_opt = screen.get_monitor_plug_name(monitor_index);
//         let has_name = monitor_name_opt.is_some();
//         let monitor_name = monitor_name_opt.unwrap_or("poop".to_string());
//         (has_name, monitor_name)
//     }
// }

// pub fn run_command(string: &str) {
//     match connect() {
//         Ok(mut connection) => {
//             connection.run_command(string).ok();
//         },
//         Err(err) => {
//             error!("running i3 command {}", err);
//         },
//     }
// }

// use i3ipc::reply::{Workspace, Workspaces};

// fn get_workspace_list(connection: &mut I3Connection) -> Vec<Workspace> {
//     connection.get_workspaces()
//         .unwrap_or(Workspaces { workspaces: Vec::new()})
//         .workspaces
// }

// fn get_workspaces<'a>(workspace_list: &'a Vec<Workspace>, show_all: bool, has_name: bool, monitor_name: String) -> Vec<&'a Workspace> {
//     let mut workspaces: Vec<&Workspace> = workspace_list
//         .iter()
//         .filter(|w| {
//             if !show_all && has_name {
//                 w.output == monitor_name
//             } else {
//                 true
//             }
//         })
//     .collect();

//     // sort by number
//     workspaces.sort_by(|a, b| a.num.cmp(&b.num));
//     workspaces
// }

// pub fn scroll_workspace(is_next: bool, monitor_index: i32) {
//     match connect() {
//         Ok(mut connection) => {

//             // get monitor name / details
//             let (has_name, monitor_name) = gtk::get_monitor_name(monitor_index);

//             // get workspace details
//             let workspace_list = get_workspace_list(&mut connection);
//             let mut workspaces = get_workspaces(&workspace_list, false, has_name, monitor_name.clone());
//             // so we can search backwards
//             if !is_next {
//                 workspaces.reverse();
//             }

//             // get focused workspace
//             let focused_opt = workspaces.iter().find(|x| x.focused);
//             if let Some(focused) = focused_opt {
//                 // get next one
//                 let next_opt = workspaces.iter().find(|x| {
//                     if is_next {
//                         x.num > focused.num
//                     } else {
//                         x.num < focused.num
//                     }
//                 });
//                 if let Some(next) = next_opt {
//                     let command = format!("workspace {}", next.name);
//                     connection.run_command(&command)
//                         .expect("something went wrong running an i3 command");
//                 }
//             }
//         },
//         Err(err) => {
//             error!("getting i3 connection {}", err);
//         },
//     }
// }
