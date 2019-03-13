use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::util::{self, write_file, read_file};

#[derive(Clone)]
pub struct Suggestions(Rc<RefCell<Data>>);

struct Data {
    programs: Vec<String>,
}

// TODO: pamac- (find next best)
// bincode?

impl Suggestions {

    pub fn load() -> Self {
        // TODO: load from history
        // TODO: merge from cache
        // TODO: add new programs to start
        let programs_path = format!("{}/programs", *crate::config::CACHE_DIR);
        let programs = match read_file(&programs_path) {
            Ok(programs) => {
                let mut cache: Vec<String> = programs.split("\n").map(|s| s.to_owned()).collect();
                // check if new programs were added
                let programs_set = util::get_programs_set();
                let cache_set: HashSet<String> = HashSet::from_iter(cache.iter().cloned());

                let new_programs = programs_set.difference(&cache_set);
                println!("{:#?}", new_programs);
                cache
            },
            Err(_) => util::get_programs_vec(),
        };

        // println!("{:#?}", &programs[..100]);
        let data = Data {
            programs,
        };
        Suggestions(Rc::new(RefCell::new(data)))
    }

    // TODO: reload_programs
    // TODO: clear_cache

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
