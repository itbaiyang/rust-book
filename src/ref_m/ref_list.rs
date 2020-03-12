use crate::ref_m::ref_list::CirList::Nill;
use crate::ref_m::ref_list::CirList::Cir;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum CirList {
  Cir(i32, RefCell<Rc<CirList>>),
  Nill
}

impl CirList {
  fn tail(&self) -> Option<&RefCell<Rc<CirList>>> {
    match self {
      Cir(_,item) => Some(item),
      Nill => None,
    }
  }
}

pub fn circle_ref() {
  let a = Rc::new(Cir(5,RefCell::new(Rc::new(Nill))));
  let b = Rc::new(Cir(10, RefCell::new(Rc::clone(&a))));

  println!("创建circle_ref_a{:?}", a);
  println!("创建circle_ref_b{:?}", b);
  // println!("{}", a);
  if let Some(x) = a.tail() {
    *x.borrow_mut() = Rc::clone(&b);
  }
  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));
}