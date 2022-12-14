#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};
use std::mem;

fn solve(_n: usize, mut v: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    v.sort_by(|&(a1, b1), &(a2, b2)| if b1 != b2 { b1.cmp(&b2) } else { a1.cmp(&a2) });
    let mut ans: Vec<(i32, i32)> = Vec::new();
    let mut last_b: i32 = -1000;
    for (a, b) in v {
        if a >= last_b {
            ans.push((a, b));
            last_b = b;
        }
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut v: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n {
        let a: i32 = io.sp();
        let b: i32 = io.ln();
        v.push((a, b));
    }
    let ans = solve(n, v);
    writeln!(io.w, "{}", ans.len()).unwrap();
    for (a, b) in ans {
        writeln!(io.w, "{} {}", a, b).unwrap();
    }
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
