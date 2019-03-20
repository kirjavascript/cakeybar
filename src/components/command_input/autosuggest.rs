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
        // TODO: pamac- (find next best)
        // TODO: add complete for just a word
        // TODO: history

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
                // history
                // TODO: slice to limit
                println!("{:#?}", (
                    &data.history[..10],
                    data.history_limit,
                ));
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
        self.0.borrow().programs.iter()
            .find(|s| s.starts_with(input))
            .map(|s| s.to_owned())
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
                // TODO: bump to top
            } else {
                // add to history
                data.history.insert(0, input);
            }

        });
    }
}
