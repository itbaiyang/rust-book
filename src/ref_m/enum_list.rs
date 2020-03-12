use std::{cell::RefCell, rc::Rc};

// pub enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }
// pub enum List {
//   Cons(i32, Rc<List>),
//   Nil,
// }
#[derive(Debug)]
pub enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}