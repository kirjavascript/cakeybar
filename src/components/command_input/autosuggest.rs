use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::env;
use std::collections::HashSet;
use crate::util::{write_file, read_file};

#[derive(Clone)]
pub struct Suggestions(Rc<RefCell<Data>>);

struct Data {
    programs: Vec<String>,
}

// TODO: pamac- (find next best)
// TODO: clear cache

fn get_installed_programs() -> Vec<String> {
    let mut programs = HashSet::new();

    env::var("PATH").unwrap_or_else(|_| "/usr/bin".to_string())
        .split(":")
        .for_each(|path| {
            if let Ok(listing) = fs::read_dir(Path::new(&path)) {
                for entry in listing {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            let is_exec = metadata.permissions()
                                .mode() & 0o111 != 0;
                            if is_exec && metadata.is_file() {
                                if let Ok(filename) = entry.file_name().into_string() {
                                    programs.insert(filename);
                                }
                            }
                        }
                    }
                }
            }
        });

    // TODO: optionally sort unique

    let programs = programs.iter().cloned().collect::<Vec<String>>();

    programs
}

impl Suggestions {

    pub fn init() -> Self {
        // TODO: load from history
        // TODO: merge from cache
        let programs_path = format!("{}/programs", *crate::config::CACHE_DIR);
        let programs = match read_file(&programs_path) {
            Ok(programs) => programs.split("\n").map(|s| s.to_owned()).collect(),
            Err(_) => get_installed_programs(),
        };
        let q = get_installed_programs();
        println!("{:#?}", q);
        let data = Data {
            programs,
        };
        Suggestions(Rc::new(RefCell::new(data)))
    }

    // TODO: reload_programs

    pub fn find(&self, input: &str) -> Option<String> {
        self.0.borrow().programs.iter()
            .find(|s| s.starts_with(input))
            .map(|s| s.to_owned())
    }

    pub fn complete(&self, input: &str) -> Option<String> {
        let position_opt = self.0.borrow().programs.iter().position(|s| {
            s.starts_with(input)
        });

        if let Some(position) = position_opt {
            // remove chosen item
            let suggestion = self.0.borrow_mut().programs.swap_remove(position);

            // add to the start of the list
            self.0.borrow_mut().programs.insert(0, suggestion.to_owned());

            self.save_cache();

            Some(suggestion)
        } else {
            None
        }
    }

    fn borrow<F>(&self, cb: F) where F: Fn(&Ref<'_, Data>) {
        cb(&self.0.borrow());
    }

    fn save_cache(&self) {
        self.borrow(|data| {
            // save program order
            let programs_text = data.programs.join("\n");
            let programs_path = format!("{}/programs", *crate::config::CACHE_DIR);

            if let Err(err) = write_file(&programs_path, &programs_text) {
                error!(
                    "tried to save program cache {}",
                    err.to_string().to_lowercase(),
                );
            }
        });
    }
}
