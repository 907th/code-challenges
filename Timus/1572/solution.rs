use std::io;
use Object::{*};

const PI: f64 = std::f64::consts::PI;

enum Object {
    Circle(usize), // 1
    Square(usize), // 2
    Triangle(usize) // 3
}

impl Object {
    fn min(&self) -> f64 {
        match *self {
            Circle(a) => a as f64 * 2.0,
            Square(a) => a as f64,
            Triangle(a) => a as f64 * (PI / 3.0).sin()
        }
    }
    fn max(&self) -> f64 {
        match *self {
            Circle(a) => a as f64 * 2.0,
            Square(a) => a as f64 * 2.0_f64.sqrt(),
            Triangle(a) => a as f64
        }
    }
}

fn read_obj() -> Object {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read obj");
    let nums: Vec<usize> = buf.trim().split(' ').map(|s| s.parse().expect("parse obj")).collect();
    if nums[0] == 1 { return Circle(nums[1]); }
    if nums[0] == 2 { return Square(nums[1]); }
    if nums[0] == 3 { return Triangle(nums[1]); }
    panic!("Unexpected obj type {}!", nums[0]);
}

fn read_num() -> usize {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read num");
    let num = buf.trim().parse().expect("parse num");
    num
}

fn main() {
    let well = read_obj();
    let mut ans = 0;
    let mut n = read_num();
    while n > 0 {
        let cover = read_obj();
        if well.max() >= cover.min() { ans += 1; }
        n -= 1;
    }
    println!("{}", ans);
}

