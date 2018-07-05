use i3ipc::reply::{Workspace as I3Workspace};

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
