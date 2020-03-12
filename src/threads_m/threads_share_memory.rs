use std::{
  sync::{Mutex, Arc},
  thread,
};

pub fn share_memory_init() {
  new_share_memory();
}

fn new_share_memory() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 1..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut x = counter.lock().unwrap();
      *x += 1;
    });
    handles.push(handle);
  }
  for i in handles {
    i.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}
