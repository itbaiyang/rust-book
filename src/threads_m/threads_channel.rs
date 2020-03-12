use std::{sync::mpsc, thread, time::Duration};

pub fn channel_init() {
  new_channel();
}

fn new_channel() {
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);
   thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });
  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("from"),
      String::from("the"),
      String::from("thread2"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });
  for received in rx {
    println!("Got: {}", received);
  }
}
