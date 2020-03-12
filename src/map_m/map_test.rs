use std::collections::HashMap;
pub fn init () {
  new_map();
}

fn new_map() {
  let mut hash_map = HashMap::new();
  hash_map.insert(String::from("id"), "1");
  hash_map.insert(String::from("name"), "baiyang");
  println!("{:?}", hash_map);

  let key = vec![String::from("id"),String::from("name")];
  let value = vec!["1","baiyang"];
  let mut user: HashMap<_,_> = key.iter().zip(value.iter()).collect();
  println!("{:?}", user);
  // let name = user.get(&String::from("name"));
  // if let Some(i) = name {
  //   println!("{:?}", i);
  // }
  let str = String::from("id");
  let sex = String::from("sex");
  user.entry(&str).or_insert(&"2");
  user.entry(&sex).or_insert(&"男");
  for (key,value) in &user {
    println!("{:?}:{:?}", key,value);
  }
  hash_map.entry(String::from("id")).or_insert("2");
  hash_map.entry(String::from("sex")).or_insert("男");
  for (key,value) in &hash_map {
    println!("{:?}:{:?}", key,value);
  }

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for i in text.split_whitespace() {
    let count = map.entry(i).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
}