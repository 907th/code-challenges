#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};
use std::mem::{swap, take, replace};

fn solve(n: usize, v: Vec<i32>) -> i32 {
    const MAX: i32 = i32::MAX;
    let pow: usize = 1 << n;
    let mut m1 = vec![MAX; pow];
    let mut m2 = vec![MAX; pow];
    let mut a = &mut m1;
    let mut b = &mut m2;
    a[0] = 0;
    let mut ans = MAX;
    for _ in 0..n {
        swap(b, &mut vec![MAX; pow]);
        for i in 0..(1 << n) {
            if a[i] == MAX { continue; }
            for j in 0..n {
                let mut s: usize = 1 << j;
                if j > 0 { s |= 1 << (j - 1); } else { s |= 1 << (n - 1); }
                if j < n - 1 { s |= 1 << (j + 1); } else { s |= 1; }
                let mut u: i32 = a[i];
                for k in 0..n {
                    if ((1 << k) & (i | s)) == 0 { u += v[k]; }
                }
                b[i | s] = min(b[i | s], u);
            }
        }
        ans = min(ans, b[pow - 1]);
        swap(&mut a, &mut b);
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let v: Vec<i32> = io.vec();
    assert!(v.len() == n);
    let ans = solve(n, v);
    writeln!(io.w, "{}", ans).unwrap();
}

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    solve_with_io(&mut io);
}

// I/O

struct IO<R: Read, W: Write> {
    r: BufReader<R>,
    w: BufWriter<W>
}

#[allow(dead_code)]
impl<R: Read, W: Write> IO<R, W> {
    fn new(r: R, w: W) -> Self {
        Self{ r: BufReader::new(r), w: BufWriter::new(w) }
    }

    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.r.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }

    fn vec<T: FromStr>(&mut self) -> Vec<T> where <T as FromStr>::Err: Debug {
        let mut buf = String::new();
        self.r.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until<T: FromStr>(&mut self, byte: u8) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
    }

    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug { self.until(0xA) }

    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug { self.until(0x20) }
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{IO, solve_with_io};

    #[test]
    fn run_tests() {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(r) => {
                    println!("Running test {}", t);
                    let w: Vec<u8> = Vec::new();
                    let mut io = IO::new(r, w);
                    solve_with_io(&mut io);
                    let out_received_bytes = io.w.into_inner().expect("Unable to unwrap output");
                    let out_received = std::str::from_utf8(&out_received_bytes).expect("Expected out is not utf8");
                    let out_expected_bytes = fs::read(out_path).expect("Unable to read out file");
                    let out_expected = std::str::from_utf8(&out_expected_bytes).expect("Expected out is not utf8");
                    assert_eq!(out_expected, out_received, "Wrong answer");
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No tests were found");
        println!("Total run {} examples", t - 1);
    }
}
