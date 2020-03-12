#[derive(Debug)]
struct User {
    name: String,
    id: i32,
    is_good: bool
}
impl User {
    fn new(name:String,id:i32) -> Self {
       User {
        name,
        id,
        is_good: false
      }
    }
    fn print_name(&self)  {
      println!("{}", &self.name);
    }
}
pub fn init_struct(name:String,id:i32) {
  let user = User::new(name, id);
  println!("{:?}", user);
  user.print_name();
}


