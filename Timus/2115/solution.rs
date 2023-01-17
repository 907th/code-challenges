#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

enum Solution {
    AlreadyOK,
    Impossible,
    Possible(usize, usize)
}

fn already_sorted(v: &Vec<i32>) -> bool {
    let n = v.len();
    for i in 1..n {
        if v[i - 1] > v[i] { return false; }
    }
    true
}

fn find_swap(v: &Vec<i32>) -> Option<(usize, usize)> {
    let n = v.len();
    let mut pair = (0usize, 0usize);
    let mut max_pos: usize = 0;
    for i in 1..n {
        if v[i] < v[max_pos] && pair.1 - pair.0 < i - max_pos {
            pair = (max_pos, i);
        }
        if v[i] > v[max_pos] {
            max_pos = i;
        }
    }
    for i in 1..n {
        let prev =
            if i - 1 == pair.0 { pair.1 }
            else if i - 1 == pair.1 { pair.0 }
            else { i - 1 };
        let this =
            if i == pair.0 { pair.1 }
            else if i == pair.1 { pair.0 }
            else { i };
        if v[prev] > v[this] { return None; }
    }
    Some(pair)
}

fn solve(mut v: Vec<i32>) -> Solution {
    let n = v.len();
    if already_sorted(&v) { return Solution::AlreadyOK; }
    if let Some(swap) = find_swap(&v) { return Solution::Possible(swap.0 + 1, swap.1 + 1); }
    v.reverse();
    if already_sorted(&v) { return Solution::AlreadyOK; }
    if let Some(swap) = find_swap(&v) { return Solution::Possible(n - swap.1, n - swap.0); }
    Solution::Impossible
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut v: Vec<i32> = Vec::new();
    for i in 0..n {
        let h = if i < n - 1 { io.sp() } else { io.ln() };
        v.push(h);
    }
    match solve(v) {
        Solution::AlreadyOK => writeln!(io.w, "Nothing to do here"),
        Solution::Impossible => writeln!(io.w, "No hope"),
        Solution::Possible(a, b) => writeln!(io.w, "Yes\n{} {}", a, b)
    }.unwrap();
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
