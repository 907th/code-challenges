#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

#[derive(Clone, Copy)]
struct Interval {
    l: i32,
    r: i32
}

fn solve(m: i32, mut v: Vec<Interval>) -> Option<Vec<Interval>> {
    v.sort_by(|a, b| a.l.cmp(&b.l));
    let mut ans: Vec<Interval> = Vec::new();
    let mut left: i32 = 0;
    let mut maxint: Option<Interval> = None;
    let mut i: usize = 0;
    while left < m {
        while i < v.len() && v[i].l <= left {
            if maxint.map_or(true, |x| x.r < v[i].r) {
                maxint = Some(v[i]);
            }
            i += 1;
        }
        if maxint.map_or(true, |x| x.r <= left) { return None; }
        let int = maxint.unwrap();
        ans.push(int);
        left = int.r;
        maxint = None;
    }
    Some(ans)
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let m: i32 = io.ln();
    let mut v: Vec<Interval> = Vec::new();
    loop {
        let (l, r) = (io.sp::<i32>(), io.ln::<i32>());
        if l == 0 && r == 0 { break; }
        v.push(Interval { l, r });
    }
    if let Some(ans) = solve(m, v) {
        writeln!(io.w, "{}", ans.len()).unwrap();
        for int in ans { writeln!(io.w, "{} {}", int.l, int.r).unwrap(); }
    } else {
        writeln!(io.w, "No solution").unwrap();
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
