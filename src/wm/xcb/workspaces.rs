use xcb_util::ewmh;
use crate::wm;
use crate::wm::workspace::Workspace;

pub fn focus_workspace(
    conn: &ewmh::Connection,
    screen_num: i32,
    workspace_name: &str,
) {
    let names_reply = ewmh::get_desktop_names(&conn, screen_num).get_reply();
    let names = match &names_reply {
        Ok(r) => r.strings(),
        Err(_) => Vec::new(),
    };
    let index = names.iter().position(|s| s == &workspace_name);
    if let Some(index) = index {
        let desktop_check = ewmh::request_change_current_desktop(
            &conn,
            screen_num,
            index as u32,
            0,
        ).request_check();
        if desktop_check.is_err() {
            warn!("failed to set workspace to {}", workspace_name);
        }
    } else {
        warn!("cannot find workspace {}", workspace_name);
    }
}

pub fn get_workspaces(
    conn: &ewmh::Connection,
    screen_num: i32,
    monitors: &Vec<(i32, i32, String)>,
) -> Vec<Workspace> {
    let current = ewmh::get_current_desktop(&conn, screen_num)
        .get_reply()
        .unwrap_or(0) as usize;
    let names_reply = ewmh::get_desktop_names(&conn, screen_num).get_reply();
    let names = match &names_reply {
        Ok(r) => r.strings(),
        Err(_) => Vec::new(),
    };

    let viewports_reply = ewmh::get_desktop_viewport(&conn, screen_num).get_reply();

    let viewports = match &viewports_reply {
        Ok(r) => r.desktop_viewports().iter()
            .map(|vp| (vp.x as i32, vp.y as i32)).collect(),
        Err(_) => Vec::new(),
    };

    let fallback_monitor = (0, 0, "[unknown]".to_string());

    let default_monitor = monitors.get(0).unwrap_or(&fallback_monitor);
    let mut workspaces = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let focused = i == current;
        let (vpx, vpy) = viewports.get(i).unwrap_or_else(|| &(0, 0));
        // get monitor data
        let (mindex, (_, _, output)) = monitors.iter()
            .enumerate()
            .find(|(_, (x, y, _))| (x, y) == (vpx, vpy))
            .unwrap_or((0, default_monitor));

        workspaces.push((name, focused, mindex, output));
    }

    // sort by monitors
    workspaces.sort_by(|a, b| a.2.cmp(&b.2));

    workspaces.into_iter()
        .enumerate()
        .map(|(i, (name, focused, _, output))| {
            Workspace {
                number: i as i32 + 1,
                name: name.to_string(),
                visible: focused == true,
                focused,
                urgent: false,
                output: output.to_string(),
            }
        })
    .collect::<Vec<Workspace>>()
}

pub fn cycle_workspace(forward: bool, monitor_index: i32) {
    match wm::xcb::connect_ewmh() {
        Ok((conn, screen_num)) => {
            let monitors = wm::gtk::get_monitor_coords();
            let workspaces = get_workspaces(&conn, screen_num, &monitors);

            let next_opt = wm::workspace::get_next(&workspaces, forward, monitor_index);

            if let Some(next) = next_opt {
                focus_workspace(&conn, screen_num, &next.name);
            }
        }
        Err(err) => {
            error!("{}", err);
        }
    }
}
