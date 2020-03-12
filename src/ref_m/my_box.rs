use std::ops::Deref;

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
  pub fn new (x:T) -> Self{
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}
