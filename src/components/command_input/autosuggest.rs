// use std::cell::{RefCell, Ref};
// use std::rc::Rc;

// // https://paste.rs/lFw.rs

// struct Suggestions(Rc<RefCell<Data>>);

// struct Data {
//     list: Vec<String>,
// }

// // macro_rules! stru! {
// //     ($name:ident $data:block) => {
// //         £name

// //     }
// // }

// macro_rules! func {
//     ($name:ident (&self, $( $i:ident: $t:ty ),*) $code:expr ) => {
//         fn $name(&self, $($i: $t),*) {
//             self.borrow(|self_| $code);
//         }
//     };
//     ($name:ident (&self) $code:expr ) => {
//         fn $name(&self) {
//             self.borrow(|self_| $code);
//         }
//     };
//     // ($name:ident (&mut self, $( $i:ident: $t:ty ),*) $code:expr ) => {
//     //     fn $name(&self, $($i: $t),*) {
//     //         self.borrow_mut(|_self| {
//     //             $code
//     //         });
//     //     }
//     // };
//     // ($name:ident (&mut self) $code:expr ) => {
//     //     fn $name(&self) {
//     //         self.borrow_mut(|_self| {
//     //             $code
//     //         });
//     //     }
//     // };
// }

// impl Suggestions {
//     fn borrow<F>(&self, cb: F) where F: Fn(&Ref<'_, Data>) {
//         cb(&self.0.borrow());
//     }

//     func!(test2(&self) {

//         self_.list.len();
//     });
// }

    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    struct Suggestions(Rc<RefCell<Data>>);
    struct Data {
        list: Vec<String>,
    }
    impl Suggestions {
        fn borrow<F>(&self, cb: F)
        where
            F: Fn(&Ref<'_, Data>),
        {
            cb(&self.0.borrow());
        }
        fn test2(&self) {
            self.borrow(|self_| {
                self_.list.len();
            });
        }
    }
