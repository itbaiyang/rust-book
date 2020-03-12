pub fn other_function(a:i32,_b:char,_c:bool,_d:(i32,char,bool),e:[i32;3],f:&String) -> i32 {
  testsuoyouq();
  testyinyong(f);
  iftest(a);
  looptest();
  whiletest();
  fortest(e);
  123
}
fn testyinyong (f:&String) {
  println!("{}",f);
}
fn testsuoyouq() {
  let x = 1;
  let y = x;
  println!("x{}y{}", x, y);
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("x{}y{}", s1, s2);
}
fn iftest(a:i32) {
  let a1 = if a < 12 {
      5
  } else {
      13
  };
  println!("{}",a1);
}
fn looptest() {
  let mut counter = 0;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  println!("The result is {}", result);
}
fn whiletest() {
  let mut a = 1;
  while a < 3 {
     a = a + 1;
  }
  println!("{}", a);
}
fn fortest(e: [i32;3]) {
  for item in e.iter() {
      println!("111{}", item);
  }
}