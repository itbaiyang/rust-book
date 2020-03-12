use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::error::Error;
pub fn init () {
  file();
  if let Ok(t) = read_from_file() {
    println!("{:?}", t);
  } else {
    println!("error");
  }
  if let Ok(t) = read_to_string_fn() {
    println!("{:?}", t);
  } else {
    println!("error");
  }
  let num = Guess::new(99);
  println!("{}", num.value());
  let num = Guess::new(199);
  println!("{:?}", num);
}

fn file() {
  let text = File::open("hello1.txt");
  let _text = match text {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound =>match File::create("hello1.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
    },
    other_error => panic!("Problem opening the file: {:?}", other_error)
    }
  };
  // println!("{:?}", text);
  File::open("hello2.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello2.txt").unwrap_or_else (|error| {
        panic!("problem creating the file {:?}", error)
      })
    } else {
      panic!("problem open the file {:?}",error)
    }
  });

  // File::open("hello4.txt").unwrap();
  // File::open("hello5.txt").expect("not found");
}
fn read_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}
fn read_to_string_fn () -> Result<(),Box<dyn Error>> {
  fs::read_to_string("hello.txt")?;
  Ok(())
}

#[derive(Debug)]
pub struct Guess {
  value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
      if value < 1 || value > 100 {
        panic!("out {}", value);
      };
      Guess {
        value
      }
    }
    pub fn value(&self) -> i32 {
      self.value
    }
}