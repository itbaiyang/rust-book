mod base_m;
mod struct_m;
mod enum_m;
mod vector_m;
mod string_m;
mod map_m;
mod panic_m;
mod t_m;
mod trait_m;
mod lifetime_m;
pub mod closures_m;
mod iter_m;
mod ref_m;
mod threads_m;
mod oop_m;
mod advanced_m;
pub use base_m::base_test;
pub use struct_m::struct_test;
pub use enum_m::enum_test;
pub use vector_m::vector_test;
pub use string_m::string_test;
pub use map_m::map_test;
pub use panic_m::panic_test;
pub use t_m::t_test;
pub use trait_m::trait_test;
pub use lifetime_m::lifetime_test;
pub use closures_m::closures_test;
pub use iter_m::iter_test;
pub use ref_m::ref_init;
pub use threads_m::threads_init;
pub use oop_m::oop_init;
pub use advanced_m::advanced_init;
use std::env;

pub fn init() {
  let args: Vec<String> = env::args().collect();
    let env = &args[1];
    // println!("{:?}", env);
    let a = 1;
    let b = 'b';
    let c = false;
    let d:(i32, char, bool) = (1,'b',false);
    let e = [1,2,3];
    let f = String::from("baiyang");
    match &env[..] {
        "base" => {
            base_test::other_function(a,b,c,d,e,&f);
        },
        "enum" => {
            enum_test::init();
        },
        "struct" => {
            struct_test::init_struct(f,a);
        },
        "vector" => {
            vector_test::init();
        },
        "string" => {
            string_test::init();
        },
        "map" => {
            map_test::init();
        },
        "panic" => {
            panic_test::init();
        },
        "t" => {
            t_test::init();
        },
        "trait" => {
            trait_test::init();

        },
        "lifetime" => {
            lifetime_test::init();
        },
        "closures" => {
            closures_test::init();
        },
        "iter" => {
            iter_test::init();
        },
        "ref" => {
            ref_init::init();
        },
        "threads" => {
            threads_init::init();
        },
        "oop" => {
            oop_init::init();
        },
        "advanced" => {
            advanced_init::init();
        },
        _ => {

        }
    }
}