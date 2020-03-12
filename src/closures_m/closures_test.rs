use std::{thread, time::Duration};

pub fn init() {
  plan();
}

fn plan() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {

  let mut expensive_closure = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_closure.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_closure.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}

#[derive(Debug)]
struct Cacher<T>
where T: Fn (u32) -> u32 
{
  calculation: T,
  value:Option<u32>,
}

impl<T> Cacher<T>
where T: Fn (u32) -> u32
{
  fn new(calculation: T) -> Self{
    Cacher {
      calculation,
      value:None,
    }
  }
  fn value(&mut self, args:u32) -> u32 {
    match self.value {
      Some(x) => x,
      None => {
        let v = (self.calculation)(args);
        self.value = Some(v);
        v
      }
    }
  }
}

#[test]
fn call_with_different_values() {
  let x = 4;

  let equal_to_x = |z| z == x;

  let y = 4;

  assert!(equal_to_x(y));
}