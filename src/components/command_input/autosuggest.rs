use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::collections::HashSet;
use std::iter::FromIterator;

use serde::{Serialize, Deserialize};

use crate::util;

#[derive(Clone)]
pub struct Suggestions(Rc<RefCell<Data>>);

#[derive(Serialize, Deserialize)]
struct Data {
    history: Vec<String>,
    history_limit: usize,
    programs: Vec<String>,
    priority_index: usize, // below which, recent programs live
}

impl Data {
    fn get_path() -> String {
        format!("{}/suggestions", *crate::config::CACHE_DIR)
    }

    fn save(&self) {
        if let Err(err) = util::write_data(&Self::get_path(), &self) {
            info!(
                "creating new program cache - {}",
                err.to_string().to_lowercase(),
            );
        }
    }
}

impl Suggestions {
    fn borrow_mut<F>(&self, cb: F) where F: Fn(&mut RefMut<'_, Data>) {
        cb(&mut self.0.borrow_mut());
    }

    pub fn load() -> Self {
        let data = match util::read_data::<Data>(&Data::get_path()) {
            Ok(mut data) => {
                let programs_set = util::get_programs_set();
                let cache_set = HashSet::from_iter(data.programs.iter().cloned());
                // check if new programs were added
                let diff_new: Vec<&String> = programs_set.difference(&cache_set).collect();
                if !diff_new.is_empty() {
                    diff_new.iter().for_each(|program| {
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
                // check if old programs were removed
                let diff_old: Vec<&String> = cache_set.difference(&programs_set).collect();
                if !diff_old.is_empty() {
                    diff_old.iter().for_each(|program| {
                        let position_opt = data.programs.iter().position(|s| {
                            &s == program
                        });
                        if let Some(position) = position_opt {
                            // remove item
                            data.programs.retain(|item| &item != program);
                            // adjust index
                            if position < data.priority_index {
                                data.priority_index -= 1;
                            }
                        }
                    });
                    data.save();
                }
                // limit history to N items
                if data.history.len() > data.history_limit {
                    data.history.drain(data.history_limit..);
                }
                data
            },
            Err(err) => {
                warn!(
                    "creating new command cache - {}",
                    err.to_string().to_lowercase(),
                );
                let data = Data {
                    history: Vec::new(),
                    history_limit: 1000,
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
        self.0.borrow().history.iter()
            .chain(self.0.borrow().programs.iter())
            .find(|s| s.starts_with(input))
            .map(|s| s.to_owned())
    }

    pub fn find_word(&self, input: &str) -> Option<String> {
        if let Some(mut suggestion) = self.find(input) {
            let rest = suggestion.split_off(input.len());
            println!("{:#?}", rest);
            None
        } else {
            None
        }
    }

    pub fn select(&self, input: &str) {
        self.borrow_mut(|data| {
            let input = input.to_string();
            if data.programs.contains(&input) {
                // if found in programs list
                let position_opt = data.programs.iter().position(|s| {
                    s == &input
                });
                if let Some(position) = position_opt {
                    // remove chosen item
                    data.programs.retain(|item| item != &input);
                    // add to start of list
                    data.programs.insert(0, input);
                    // increase priority index if needed
                    if position >= data.priority_index {
                        data.priority_index += 1;
                    }
                    data.save();
                }
            } else if data.history.contains(&input) {
                // bump history item to top
                data.history.retain(|item| item != &input);
                data.history.insert(0, input);
                data.save();
            } else {
                // add to history
                data.history.insert(0, input);
                // ensure we dont hit the limit
                let limit = data.history_limit;
                if data.history.len() > limit {
                    data.history.drain(limit..);
                }
                data.save();
            }

        });
    }
}
