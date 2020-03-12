pub fn init () {
  new_vector();
  enum_vector();
}

#[derive(Debug)]
enum MultiType {
    Int(i32),
    Float(f32),
    Text(String),
}
fn new_vector() {
  let mut v = vec![1,2,3];
  v.push(4);
  match v.get(2) {
    Some(third) => println!("{}", third),
    None => println!("None"),
  }

  for i in &v {
      println!("{:?}", i);
  }
}
fn enum_vector () {
  let v = vec![MultiType::Int(12),MultiType::Float(0.12),MultiType::Text(String::from("baiyang"))];
  match v.get(2) {
    Some(MultiType::Text(i)) => {
      println!("数据字符串类型，值是：{:?}",i);
    },
    Some(MultiType::Float(i)) => {
      println!("数据浮点类型，值是：{:?}",i);
    },
    Some(MultiType::Int(i)) => {
      println!("数据整数类型，值是：{:?}",i);
    },
    None => println!("none"),
  }
}