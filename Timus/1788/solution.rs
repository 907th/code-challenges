#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::{self, FromStr};
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};

fn solve(n: usize, m: usize, mut g: Vec<i32>, mut b: Vec<i32>) -> i32 {
    g.sort(); g.reverse();
    b.sort(); b.reverse();
    let mut girls: i32 = g.iter().sum();
    let mut boys: i32 = b.iter().sum();
    let mut ans = girls + boys * 0;
    for i in 0..min(n, m) {
        girls -= g[i];
        boys -= b[i];
        ans = min(ans, girls + boys * (i as i32 + 1));
    }
    ans
}

#[allow(unused_must_use)]
fn solve_with_io<R: Read>(inp: &mut Input<R>, out: &mut dyn Write) {
    let n: usize = inp.sp();
    let m: usize = inp.ln();
    let g: Vec<i32> = inp.vec();
    let b: Vec<i32> = inp.vec();
    assert!(g.len() == n);
    assert!(b.len() == m);
    let ans = solve(n, m, g, b);
    writeln!(out, "{}", ans).unwrap();
}

fn main() {
    let mut inp = Input::new(io::stdin());
    let mut out = io::stdout();
    solve_with_io(&mut inp, &mut out);
}

// I/O

trait InputRead<T> {
    fn chr(&mut self) -> char;
    fn vec(&mut self) -> Vec<T>;
    fn until(&mut self, byte: u8) -> T;
    fn ln(&mut self) -> T { self.until(0xA) }
    fn sp(&mut self) -> T { self.until(0x20) }
}

struct Input<R>(BufReader<R>);

impl<R: Read> Input<R> {
    fn new(r: R) -> Self {
        Self(BufReader::new(r))
    }
}

impl<R: Read, T: FromStr> InputRead<T> for Input<R>
where <T as FromStr>::Err: Debug {
    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.0.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }

    fn vec(&mut self) -> Vec<T> {
        let mut buf = String::new();
        self.0.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until(&mut self, byte: u8) -> T {
        let mut buf = Vec::new();
        self.0.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
    }
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{Input, solve_with_io};

    #[test]
    fn run_tests() {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(in_file) => {
                    let mut inp = Input::new(in_file);
                    let mut out: Vec<u8> = Vec::new();
                    solve_with_io(&mut inp, &mut out);
                    let out_expected = fs::read(out_path).expect("Unable to read out file");
                    assert_eq!(
                        out, out_expected,
                        "Wrong answer:\n{}\nExpected:\n{}\n",
                        std::str::from_utf8(&out).unwrap(),
                        std::str::from_utf8(&out_expected).unwrap()
                    );
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No tests were found");
    }
}
