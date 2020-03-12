// use core::fmt::Display;
use std::cmp::PartialOrd;
pub fn init() {
  let t_int = vec![1,2,3,4,5,4,3,2,1];
  let t_char = vec!['a', 'c', 'd', 'b'];
  // get_largest(&t_int);
  // get_largest(&t_char);
  println!("{:?}", get_largest(&t_int));
  println!("{:?}", get_largest(&t_char));
  let wont_work = Point { x: 5, y: 4 };
  println!("{}", wont_work.x());
}

// fn get_largest<T:PartialOrd + Clone>(list: &[T]) -> T {
//   let mut largest = list[0].clone();
//   for item in list.clone().iter() {
//     if item > &largest {
//       largest = item.clone();
//     };
//   }
//   largest
// }
fn get_largest<T:PartialOrd>(list: &[T]) -> &T {
  let slices = &list[..];
  let mut largest = &slices[0];
  for item in slices.iter() {
    if item > largest {
      largest = item;
    };
  }
  &largest
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }
  // fn area(&self) -> U{
  //   &self.x * &self.y
  // }
    // add code here
}
// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }