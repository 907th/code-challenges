#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

fn solve(_k: usize, v: Vec<(usize, usize)>, n: usize) -> Vec<usize> {
    let half = n / 2;
    let mut ans: Vec<usize> = Vec::new();
    let mut sum: usize = 0;
    let mut k0: usize = 0;
    let mut n0: usize = 0;
    let mut k1: usize = 0;
    let mut n1: usize = 0;
    while sum < half {
        if sum + v[k1].1 <= half {
            sum += v[k1].1;
            k1 += 1;
        } else {
            n1 += half - sum;
            break;
        }
    }
    while ans.len() < n {
        ans.push(v[k1].0);
        n1 += 1;
        if n1 >= v[k1].1 {
            k1 += 1;
            n1 = 0;
        }
        if ans.len() < n {
            ans.push(v[k0].0);
            n0 += 1;
            if n0 >= v[k0].1 {
                k0 += 1;
                n0 = 0;
            }
        }
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let k: usize = io.ln();
    let mut v: Vec<(usize, usize)> = Vec::new();
    let mut n: usize = 0;
    for i in 0..k {
        let x: usize = if i < k - 1 { io.sp() } else { io.ln() };
        v.push((i + 1, x));
        n += x;
    }
    v.sort_by(|a, b| a.1.cmp(&b.1));
    let ans = solve(k, v, n);
    assert!(ans.len() == n, "Invalid ans length");
    for i in 0..n {
        write!(io.w, "{}", ans[i]).unwrap();
        write!(io.w, "{}", if i < n - 1 { " " } else { "\n" }).unwrap();
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
