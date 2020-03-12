pub fn init () {
  new_string();
}

fn new_string() {
  let mut str = "baiyang".to_string();
  str.push_str(" hello");
  println!("{:?}",str);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{:?}",s);
  let chinese = "中文";
  let s_zh = &chinese[0..3];
  println!("{:?}", s_zh);
  for i in chinese.chars() {
    println!("{:?}", i);
  }
  for i in chinese.bytes() {
    println!("{}", i);
  }
}