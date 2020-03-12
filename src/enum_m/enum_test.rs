#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}
#[derive(Debug)]
enum RMB {
    Yuan(State),
    Mao,
    Fen,
}
#[derive(Debug)]
enum State {
    Y1,
    _Y2,
}
fn value_rmb(rmb: RMB) -> u8{
  match rmb {
    RMB::Yuan(_state) => 100,
    RMB::Mao => 10,
    RMB::Fen => 1,
  }
}
pub fn init () {
  let four = IpAddr::V4(127,0,0,1);
  let six = IpAddr::V6(String::from("::1"));
  route(four);
  route(six);
  let rmb:RMB = RMB::Mao;
  let rmb_f:RMB = RMB::Fen;
  let rmb_y:RMB = RMB::Yuan(State::Y1);
  let r = value_rmb(rmb);
  value_rmb(rmb_f);
  value_rmb(rmb_y);
  println!("{}",r);
  let s = Some(5);
  option_test(s);
  option_test(None);
  if_let_test();
}
fn route(ip: IpAddr) -> IpAddr {
  println!("{:?}", ip);
  ip
}

fn option_test(op : Option<i32>) -> Option<i32> {
  match op {
   
    Some(i) => {
      println!("Some {}", i);
      Some(i+1)
    },
    _ => {
      println!("None");
      None
    },
  }
}
fn if_let_test() -> bool{
  let temp = Some(3);
  if let Some(3) = temp {
    println!("{:?}",temp);
  }
  true
}