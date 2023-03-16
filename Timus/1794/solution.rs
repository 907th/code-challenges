#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

fn solve(n: usize, v: Vec<usize>) -> usize {
    let mut k = vec![0usize; n];
    for (i, x) in v.iter().enumerate() {
        k[(n + i - (x - 1)) % n] += 1;
    }
    k.iter().enumerate().max_by_key(|p| p.1).unwrap().0 + 1
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let v: Vec<usize> = io.lnsp(n);
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
    fn lnsp<T: FromStr>(&mut self, n: usize) -> Vec<T> where <T as FromStr>::Err: Debug {
        (0..n).map(|i| if i < n - 1 { self.sp() } else { self.ln() }).collect()
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
                    println!("Example {}", t);
                    let w: Vec<u8> = Vec::new();
                    let mut io = IO::new(r, w);
                    solve_with_io(&mut io);
                    let out_received_bytes = io.w.into_inner().unwrap();
                    let out_received = std::str::from_utf8(&out_received_bytes).unwrap();
                    let out_expected_bytes = fs::read(out_path).unwrap();
                    let out_expected = std::str::from_utf8(&out_expected_bytes).unwrap();
                    assert_eq!(out_expected, out_received, "Wrong answer");
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No examples were found");
        println!("Total run {} examples", t - 1);
    }
}
