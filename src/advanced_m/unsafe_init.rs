// 解引用裸指针
// 调用不安全的函数或方法
// 访问或修改可变静态变量
// 实现不安全 trait
// 访问 union 的字段
use std::slice;
pub fn init() {
  luozhizhen();
  unsafe_object();
  extern_unsaft();
  changestatic(12);
}

// 解引用裸指针
// 与引用和智能指针的区别在于，记住裸指针

//     允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
//     不保证指向有效的内存
//     允许为空
//     不能实现任何自动清理功能

fn luozhizhen() {
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("{}", *r1);
    println!("{}", *r2);
    dangerous();
  }
}

// 调用不安全函数或方法
unsafe fn dangerous() {}

// 创建不安全代码的安全抽象
fn unsafe_object() {
  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = r.split_at_mut(3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);
}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//   let len = slice.len();

//   assert!(mid <= len);

//   (&mut slice[..mid],
//    &mut slice[mid..])
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
    )
  }
}
// 使用 extern 函数调用外部代码
extern "C" {
  fn abs(input: i32) -> i32;
}

fn extern_unsaft() {
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

// 访问或修改可变静态变量
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn changestatic(inc: u32) {

  unsafe {
    COUNTER += inc;
    println!("COUNTER: {}", COUNTER);
  }
  
  println!("name is: {}", HELLO_WORLD);
}
// 实现不安全 trait