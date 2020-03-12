use std::{thread, time::Duration};
use super::{threads_share_memory::share_memory_init, threads_channel::channel_init};

pub fn init() {
  new_thread();
  channel_init();
  share_memory_init();
}
fn new_thread() {
  // let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    for _i in 0..3 {
      // println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
      // println!("{}",&v[i]);
    }
  });
  handle.join().unwrap();
  // drap(v);
  for _i in 1..5 {
    // println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  // handle.join().unwrap();
}
