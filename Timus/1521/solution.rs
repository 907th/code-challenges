#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

struct BinaryIndexedTree {
    data: Vec<i32>
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self { data: vec![0; n + 1] }
    }

    // Least significant bit
    fn lsb(i: usize) -> usize {
        i & ((usize::MAX ^ i) + 1)
    }

    // Add 'add_value' to cell [i]
    fn add(&mut self, mut i: usize, add_value: i32) {
        assert!(i > 0, "array indices are 1-based");
        let n = self.data.len();
        while i < n {
            self.data[i] += add_value;
            i += Self::lsb(i);
        }
    }

    // Sum on interval [1, i]
    fn prefix_sum(&self, mut i: usize) -> i32 {
        assert!(i > 0, "array indices are 1-based");
        let mut s = 0;
        while i > 0 {
            s += self.data[i];
            i -= Self::lsb(i);
        }
        s
    }

    // Find smallest index [i] such that sum on [1, n] is <= 'target_sum'
    fn find(&self, target_sum: i32) -> usize {
        let mut l = 1;
        let mut r = self.data.len();
        while l < r {
            let c = (l + r) / 2;
            if self.prefix_sum(c) < target_sum {
                l = c + 1;
            } else {
                r = c;
            }
        }
        l
    }
}

fn solve(n: usize, k: usize) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();
    let mut b = BinaryIndexedTree::new(n);
    for i in 1..=n { b.add(i, 1); }
    let mut j = 0;
    for i in 0..n {
        j = (j + k - 1) % (n - i);
        let v = b.find(j as i32 + 1);
        ans.push(v);
        b.add(v, -1);
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.sp();
    let k: usize = io.ln();
    let ans = solve(n, k);
    for (i, v) in ans.iter().enumerate() {
        if i > 0 { write!(io.w, " {}", v) }
        else { write!(io.w, "{}", v) }.unwrap();
    }
    writeln!(io.w, "").unwrap();
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

    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n];
        self.r.read_exact(&mut buf).expect("Unable to read exact number of bytes");
        buf
    }

    fn byte(&mut self) -> u8 { self.bytes(1)[0] }

    fn parse_until<T: FromStr>(&mut self, byte: u8, trim: &[char]) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not valid utf8");
        str.trim_end_matches(trim).parse().expect("Unable to parse string")
    }

    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
    }
    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0x20, &[0x20 as char])
    }
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
