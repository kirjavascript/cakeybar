mod listen;

use wm;
use wm::workspace::{Workspace, i3_to_generic};

// reexported public interface
pub use self::listen::listen;

// utils
use i3ipc::{I3Connection, EstablishError};
use i3ipc::reply::{Workspaces as I3Workspaces};

pub fn connect() -> Result<I3Connection, EstablishError> {
    I3Connection::connect()
}

pub fn run_command(string: &str) {
    match connect() {
        Ok(mut connection) => {
            connection.run_command(string).ok();
        },
        Err(err) => {
            error!("running i3 command {}", err);
        },
    }
}

pub fn get_workspaces(connection: &mut I3Connection) -> Vec<Workspace> {
    let mut i3workspaces = connection.get_workspaces()
        .unwrap_or(I3Workspaces { workspaces: Vec::new()})
        .workspaces;
    // sort by number
    i3workspaces.sort_by(|a, b| a.num.cmp(&b.num));
    // convert to generic workspace
    let workspaces = i3workspaces
        .iter()
        .map(i3_to_generic)
        .collect::<Vec<Workspace>>();

    workspaces
}

pub fn cycle_workspace(is_next: bool, monitor_index: i32) {
    match connect() {
        Ok(mut connection) => {

            // get monitor name
            let name_opt = wm::gtk::get_monitor_name(monitor_index);

            // get workspace details
            let workspaces = get_workspaces(&mut connection);
            let mut workspaces = workspaces
                .iter()
                .filter(|w| {
                    match name_opt {
                        Some(ref name) => *name == w.output,
                        None => true,
                    }
                })
                .collect::<Vec<&Workspace>>();

            // so we can search backwards
            if !is_next {
                workspaces.reverse();
            }

            // get focused workspace
            let focused_opt = workspaces.iter().find(|x| x.focused);
            if let Some(focused) = focused_opt {
                // get next one
                let next_opt = workspaces.iter().find(|x| {
                    if is_next {
                        x.number > focused.number
                    } else {
                        x.number < focused.number
                    }
                });
                if let Some(next) = next_opt {
                    let command = format!("workspace {}", next.name);
                    let command_res = connection.run_command(&command);
                    if let Err(_) = command_res {
                        error!("i3: Something went wrong switching workspaces");
                    }
                }
            }
        },
        Err(err) => {
            error!("getting i3 connection {}", err);
        },
    }
}
