use std::cell::RefCell;
use std::rc::Rc;

// use bidule::Stream;

#[derive(Debug)]
pub enum Event {
    Window(String),
    Mode(String),
}

#[derive(Debug)]
pub struct EventStream {
    // data: Rc<RefCell<Data>>,
}

// #[derive(Debug)]
// struct Data {
//     stream: Stream<'static, Event>,
// }

// impl EventStream {
//     pub fn new() -> Self {

//         let data = Rc::new(RefCell::new(Data {
//             stream: Stream::new(),
//         }));

//         Self { data }
//     }

//     pub fn clone(&self) -> Self {
//         Self { data: self.data.clone() }
//     }
// }
