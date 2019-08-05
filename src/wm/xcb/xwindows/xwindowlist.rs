// use std::collections::HashMap;
// use super::XWindowData;
// use std::ops::Index;

// pub struct XWindowList(pub HashMap<xcb::Window, XWindowData>);

// #[derive(Debug)]
// pub enum XWindowEvent {
//     Add(xcb::Window, XWindowData),
//     Remove(xcb::Window),
//     Geometry(xcb::Window, (i16, i16, u16, u16)),
//     Name(xcb::Window, String),
//     Visible(xcb::Window, bool),
// }

// impl Index<&xcb::Window> for XWindowList {
//     type Output = XWindowData;

//     fn index(&self, window: &xcb::Window) -> &Self::Output {
//         &self.0[window]
//     }
// }

// impl XWindowList {
//     pub fn new() -> Self {
//         Self(HashMap::new())
//     }

//     pub fn update(&mut self, next: Self) -> Vec<(xcb::Window, XWindowEvent)> {
//         let mut events = Vec::new();
//         for (xwindow, xwindowdata) in next.0.iter() {
//             if let Some(prev_xwindowdata) = self.0.get(&xwindow) {
//                 let XWindowData {
//                     x, y, width, height, name, visible,
//                 } = &prev_xwindowdata;
//                 if name != &xwindowdata.name {
//                     events.push((*xwindow, XWindowEvent::Name));
//                 }
//                 if visible != &xwindowdata.visible {
//                     events.push((*xwindow, XWindowEvent::Visible));
//                 }
//                 let geom_next = (
//                     &xwindowdata.x,
//                     &xwindowdata.y,
//                     &xwindowdata.width,
//                     &xwindowdata.height,
//                 );
//                 if (x, y, width, height) != geom_next {
//                     events.push((*xwindow, XWindowEvent::Geometry));
//                 }
//             } else {
//                 events.push((*xwindow, XWindowEvent::Add));
//             }
//         }

//         self.0.iter()
//             .filter(|(k, _)| !next.0.keys().any(|kk| &kk == k))
//             .for_each(|(xwindow, _)| {
//                 events.push((*xwindow, XWindowEvent::Remove));
//             });

//         // update state
//         self.0 = next.0;

//         events
//     }
// }
