// use crate::box_m::enum_list::List::{Cons, Nil};
use super::enum_list::List::{Cons, Nil};
use super::my_box::MyBox;
use super::{tree_list::tree_init, ref_list::circle_ref};
use std::{cell::RefCell, rc::Rc};
pub fn init() {
  // new();
  // enum_box();
  my_box();
  refcell();
  circle_ref();
  tree_init();
}
// fn new() {
//   let b = Box::new(5);
//   println!("{:?}", b);
// }

// fn enum_box() {
//   // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

fn my_box() {
  let x = 5;
  let y = MyBox::new(x);
  let m = MyBox::new(String::from("first"));
  hello(&m);
  assert_eq!(5, x);
  assert_eq!(5, *y);
}
fn hello(name: &str) {
  println!("Hello, {}!", name);
}

// fn rc() {
//   let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
//   println!("count after creating a = {}", Rc::strong_count(&a));
//   let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//   println!("count after creating b = {}", Rc::strong_count(&a));
//   {
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
//     println!("count after creating c = {}", Rc::strong_count(&a));
//   }
//   println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

fn refcell() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}
