#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

fn solve(n: usize, v: Vec<i32>) -> i64 {
    let mut pos: Vec<(i32, usize)> = Vec::new();
    for i in 0..n { pos.push((v[i], i)); }
    pos.sort();
    let mut dp_l: i64 = 0;
    let mut dp_r: i64 = 0;
    let mut pos_l: usize = 0;
    let mut pos_r: usize = 0;
    let mut l: usize = 0;
    while l < n {
        let mut r = l;
        while r + 1 < n && pos[l].0 == pos[r + 1].0 { r += 1; }

        let new_pos_l = pos[l].1;
        let new_pos_r = pos[r].1;

        let new_dp_l_1: i64 = dp_l + (pos_l as i64 - new_pos_r as i64).abs() + (new_pos_r as i64 - new_pos_l as i64);
        let new_dp_l_2: i64 = dp_r + (pos_r as i64 - new_pos_r as i64).abs() + (new_pos_r as i64 - new_pos_l as i64);

        let new_dp_r_1: i64 = dp_l + (pos_l as i64 - new_pos_l as i64).abs() + (new_pos_r as i64 - new_pos_l as i64);
        let new_dp_r_2: i64 = dp_r + (pos_r as i64 - new_pos_l as i64).abs() + (new_pos_r as i64 - new_pos_l as i64);

        dp_l = min(new_dp_l_1, new_dp_l_2);
        dp_r = min(new_dp_r_1, new_dp_r_2);
        pos_l = new_pos_l;
        pos_r = new_pos_r;

        l = r + 1;
    }
    min(dp_l, dp_r) + n as i64
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut v: Vec<i32> = Vec::new();
    for i in 0..n {
        let x: i32 = if i < n - 1 { io.sp() } else { io.ln() };
        v.push(x);
    }
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
