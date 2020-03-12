/**
 * 迭代器是惰性的
 * 迭代器消费方法 next() collect()
 * 迭代器生成其他迭代器方法  map filter
*/

pub fn init() {
    // lazy_iter()
    
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);

}
#[test]
fn lazy_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // for val in v1_iter {
    //   println!("Got: {}",val);
    // }

    // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);

    // let total:i32 = v1_iter.sum();
    // assert_eq!(total, 6)

    let v2: Vec<i32> = v1_iter.map(|x| x + 1).collect();
    println!("{:?}", v2);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[derive(Debug)]
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    // 迭代器会产生 u32s
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // count 自增 1。也就是为什么从 0 开始。
        self.count += 1;
        // 检测是否结束结束计数。
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn name() {
    // let mut counter = Counter::new().zip(Counter::new().skip(1));
    // 0,1 1,2 2,3 3,4 4,5 2 6 12 20   6 12
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
