use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;
use std::iter::FromIterator;

use serde::{Serialize, Deserialize};

use crate::util;

#[derive(Clone)]
pub struct Suggestions(Rc<RefCell<Data>>);

#[derive(Serialize, Deserialize)]
struct Data {
    programs: Vec<String>,
    priority_index: usize, // below which, recent programs live
}

impl Data {
    fn get_path() -> String {
        format!("{}/suggestions", *crate::config::CACHE_DIR)
    }

    fn save(&self) {
        if let Err(err) = util::write_data(&Self::get_path(), &self) {
            error!(
                "tried to save program cache {}",
                err.to_string().to_lowercase(),
            );
        }
    }
}

impl Suggestions {
    // fn borrow<F>(&self, cb: F) where F: Fn(&Ref<'_, Data>) {
    //     cb(&self.0.borrow());
    // }

    pub fn load() -> Self {
        // TODO: pamac- (find next best)
        // TODO: load from history
        // TODO: remove uninstalled
        // TODO: only save when actually used, not just completion?
        // TODO: fix ordering bug (swap_remove)

        let data = match util::read_data::<Data>(&Data::get_path()) {
            Ok(mut data) => {
                // check if new programs were added
                let programs_set = util::get_programs_set();
                let cache_set = HashSet::from_iter(data.programs.iter().cloned());
                let difference: Vec<&String> = programs_set.difference(&cache_set).collect();
                if !difference.is_empty() {
                    difference.iter().for_each(|program| {
                        let sorted_slice = &data.programs[data.priority_index..];
                        let search_res = sorted_slice.binary_search(&program);
                        // if not found...
                        if let Err(index) = search_res {
                            // add priority index to get absolute index
                            let index = index + data.priority_index;
                            data.programs.insert(index, program.to_string());
                        }
                    });
                    data.save();
                }
                println!("{:#?}", (
                    data.priority_index,
                    &data.programs[..20],
                ));
                data
            },
            Err(err) => {
                warn!(
                    "creating new command cache - {}",
                    err.to_string().to_lowercase(),
                );
                let data = Data {
                    programs: util::get_programs_vec(),
                    priority_index: 0,
                };
                data.save();
                data
            },
        };

        Suggestions(Rc::new(RefCell::new(data)))
    }

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

            // increase priority index if needed
            if position >= self.0.borrow().priority_index {
                self.0.borrow_mut().priority_index += 1;
            }

            self.0.borrow().save();

            Some(suggestion)
        } else {
            None
        }
    }
}
