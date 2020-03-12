pub fn init() {
  let sex = "男";
  let my_sex = new_map(sex);
  println!("{:?}", my_sex);

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = LifeTimes { part: first_sentence };
  i.ann("baiyang");
}
fn new_map<'a>(sex: &'a str) -> &'a str {
  sex
}
#[derive(Debug)]
pub struct LifeTimes<'a> {
  part: &'a str,
}

impl<'a> LifeTimes<'a> {
  fn ann(&self,announcement: &str) -> &str {
    println!("Attention please: {:?}", announcement);
    self.part
  }
}