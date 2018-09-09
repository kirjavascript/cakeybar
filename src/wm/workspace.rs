use i3ipc::reply::Workspace as I3Workspace;
use wm;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub number: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: String,
}

pub fn i3_to_generic(i3workspace: &I3Workspace) -> Workspace {
    Workspace {
        number: i3workspace.num,
        name: i3workspace.name.clone(),
        visible: i3workspace.visible,
        focused: i3workspace.focused,
        urgent: i3workspace.urgent,
        output: i3workspace.output.clone(),
    }
}

pub fn get_next(
    workspaces: &Vec<Workspace>,
    forward: bool,
    monitor_index: i32,
) -> Option<&Workspace> {
    // get monitor name
    let name_opt = wm::gtk::get_monitor_name(monitor_index);
    let mut workspaces = workspaces
        .iter()
        .filter(|w| match name_opt {
            Some(ref name) => *name == w.output,
            None => true,
        })
        .collect::<Vec<&Workspace>>();

    // so we can search backwards
    if !forward {
        workspaces.reverse();
    }

    // get focused workspace
    let focused_opt = workspaces.iter().find(|x| x.focused);
    if let Some(focused) = focused_opt {
        // get next one
        let next_opt = workspaces.iter().find(|x| {
            if forward {
                x.number > focused.number
            } else {
                x.number < focused.number
            }
        });

        if let Some(next) = next_opt {
            return Some(next);
        }
    }

    None
}
