use std::io;
use std::fmt::Debug;
use std::str::FromStr;
use std::collections::BinaryHeap;

fn main() {
    let n: usize = read_num();
    let mut nums: BinaryHeap<i32> = BinaryHeap::new();

    let m = (n - 1) / 2;
    nums.reserve_exact(m + 1);
    for _ in 0..=m {
        let v = read_num();
        nums.push(v);
    }

    let mut left_max: i32 = *nums.peek().unwrap();
    let mut right_min: i32 = -1;
    for _ in (m + 1)..n {
        let v = read_num();
        if v < left_max {
            right_min = left_max;
            nums.pop();
            nums.push(v);
            left_max = *nums.peek().unwrap();
        } else {
            if right_min == -1 || right_min > v {
                right_min = v;
            }
        }
    }

    let ans: f64 =
        if n & 1 == 1 {
            left_max as f64
        } else {
            (left_max as f64 + right_min as f64) / 2.0
        };
    println!("{:.1}", ans);
}

fn read_num<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read number");
    buf.trim().parse::<T>().expect("parse number")
}
