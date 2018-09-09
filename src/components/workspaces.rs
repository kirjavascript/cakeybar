use bar::Bar;
use components::Component;
use config::ConfigGroup;
use glib::markup_escape_text;
use glib::signal::SignalHandlerId;
use gtk;
use gtk::prelude::*;
use gtk::{EventBox, Label, LabelExt, Orientation, StyleContextExt, WidgetExt};

use util::SymbolFmt;
use wm;
use wm::events::{Event, EventId, EventValue};
use wm::workspace::Workspace;
use wm::WMUtil;

use std::cell::RefCell;
use std::mem::replace;
use std::rc::Rc;

pub struct Workspaces {
    config: ConfigGroup,
    wrapper: gtk::Box,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for Workspaces {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        self.wrapper.show();
    }
    fn hide(&self) {
        self.wrapper.hide();
    }
    fn destroy(&self) {
        self.wm_util
            .remove_listener(Event::Workspace, self.event_id);
        self.wrapper.destroy();
    }
}

impl Workspaces {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let monitor_index = bar.config.get_int_or("monitor", 0) as i32;

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // misc config
        let show_all = config.get_bool_or("show-all", false);
        let symbols = SymbolFmt::new(config.get_str_or("format", "{number}"));

        // attach wrapper
        let wrapper = gtk::Box::new(Orientation::Horizontal, spacing);

        // add to container and show
        super::init_widget(&wrapper, &config, bar, container);
        wrapper.show();

        let name_opt = wm::gtk::get_monitor_name(monitor_index);

        let workspaces = bar.wm_util.get_workspaces().unwrap_or(vec![]);
        let workspaces = filter_by_name(&workspaces, show_all, &name_opt);

        // create initial UI

        let elabels: Rc<RefCell<Vec<EventLabel>>> = Rc::new(RefCell::new(Vec::new()));

        let wm_util = bar.wm_util.clone();

        for workspace in workspaces.iter() {
            let elabel = EventLabel::new(&wrapper, &wm_util, &workspace, &symbols);
            elabels.borrow_mut().push(elabel);
        }
        wrapper.show_all();

        // listen for events
        let event_id = wm_util.add_listener(Event::Workspace,
            clone!((wrapper, elabels, wm_util) move |workspaces_opt| {
                if let Some(EventValue::Workspaces(workspaces)) = workspaces_opt {

                    let mut workspaces = filter_by_name(&workspaces, show_all, &name_opt);

                    for (i, workspace) in workspaces.iter().enumerate() {
                        let added_new = if let Some(elabel) = elabels.borrow_mut().get_mut(i) {
                            // if a label already exists
                            elabel.update(&workspace, &symbols, &wm_util);
                            None
                        } else {
                            // otherwise create a new label
                            Some(EventLabel::new(
                                &wrapper,
                                &wm_util,
                                &workspace,
                                &symbols
                            ))
                        };
                        if let Some(added) = added_new {
                            elabels.borrow_mut().push(added);
                        }
                    }
                    wrapper.show_all();

                    // remove items
                    let work_len = workspaces.len();
                    let label_len = elabels.borrow().len();
                    if label_len > work_len {
                        let mut labels = elabels.borrow_mut();
                        labels.splice(work_len..label_len, vec![]).for_each(|el| {
                            el.destroy();
                        });
                    }
                }
            }
        ),
        );

        let wm_util = bar.wm_util.clone();
        bar.add_component(Box::new(Workspaces {
            config,
            wrapper,
            wm_util,
            event_id,
        }));
    }
}

struct EventLabel {
    ebox: EventBox,
    label: Label,
    event_id: SignalHandlerId,
}

impl EventLabel {
    pub fn new(
        wrapper: &gtk::Box,
        wm_util: &wm::WMUtil,
        workspace: &Workspace,
        symbols: &SymbolFmt,
    ) -> Self {
        let label = Label::new(None);
        let ebox = EventBox::new();
        ebox.add(&label);
        wrapper.add(&ebox);
        set_label_attrs(&label, workspace, &symbols);
        // attach initial event and store the id
        let workspace_name = workspace.name.to_string();
        let event_id = ebox.connect_button_press_event(clone!(wm_util
            move |_, _| {
                wm_util.focus_workspace(&workspace_name);
                Inhibit(false)
            }
        ));
        EventLabel {
            ebox,
            label,
            event_id,
        }
    }

    pub fn update(&mut self, workspace: &Workspace, symbols: &SymbolFmt, wm_util: &wm::WMUtil) {
        set_label_attrs(&self.label, workspace, symbols);
        // add a new event
        let workspace_name = workspace.name.to_string();
        let event_id = self.ebox.connect_button_press_event(clone!(wm_util
            move |_, _| {
                wm_util.focus_workspace(&workspace_name);
                Inhibit(false)
            }
        ));
        // remove the old event and update the ID
        self.ebox.disconnect(replace(&mut self.event_id, event_id));
    }

    pub fn destroy(&self) {
        self.ebox.destroy();
    }
}

fn get_set_class(ctx: gtk::StyleContext) -> impl Fn(&str, bool) {
    move |s, b| {
        if b {
            StyleContextExt::add_class(&ctx, s);
        } else {
            StyleContextExt::remove_class(&ctx, s);
        }
    }
}

fn set_label_attrs(label: &Label, workspace: &Workspace, symbols: &SymbolFmt) {
    label.set_label(&symbols.format(|sym| match sym {
        "name" => markup_escape_text(&workspace.name),
        "number" => workspace.number.to_string(),
        _ => sym.to_string(),
    }));
    // style
    if let Some(ctx) = label.get_style_context() {
        let set_class = get_set_class(ctx);
        set_class("focused", workspace.focused);
        set_class("visible", workspace.visible);
        set_class("urgent", workspace.urgent);
    }
}

fn filter_by_name<'a>(
    workspaces: &'a Vec<Workspace>,
    show_all: bool,
    name_opt: &Option<String>,
) -> Vec<&'a Workspace> {
    workspaces
        .iter()
        .filter(|w| {
            if show_all {
                true
            } else {
                match name_opt {
                    Some(ref name) => *name == w.output,
                    None => true,
                }
            }
        })
        .collect::<Vec<&Workspace>>()
}
