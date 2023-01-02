#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Mul, Add, Shl};
use std::mem;

// A value of the polynomail rolling hash function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hash(u64, u64);

impl Hash {
    const B0: u64 = 10007;
    const B1: u64 = 10009;
    const P0: u64 = 1000000093;
    const P1: u64 = 1000000097;

    fn new(v: u64) -> Self { Self(v, v) }
    fn zero() -> Self { Self::new(0) }
    fn one() -> Self { Self::new(1) }
}

impl Add for Hash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0.checked_add(other.0).unwrap() % Self::P0,
            self.1.checked_add(other.1).unwrap() % Self::P1
        )
    }
}

impl Mul for Hash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0.checked_mul(other.0).unwrap() % Self::P0,
            self.1.checked_mul(other.1).unwrap() % Self::P1
        )
    }
}

impl Shl<usize> for Hash {
    type Output = Self;
    // x << rhs means x * BASE^rhs (implemented with fast power algorithm).
    // Complexity of this operation is O(log(rhs)).
    fn shl(self, rhs: usize) -> Self {
        let mut i = rhs;
        let mut b0 = Self::B0;
        let mut b1 = Self::B1;
        let mut v0 = self.0;
        let mut v1 = self.1;
        while i > 0 {
            if i & 1 == 1 {
                v0 = v0.checked_mul(b0).unwrap() % Self::P0;
                v1 = v1.checked_mul(b1).unwrap() % Self::P1;
            }
            b0 = b0.checked_mul(b0).unwrap() % Self::P0;
            b1 = b1.checked_mul(b1).unwrap() % Self::P1;
            i = i >> 1;
        }
        Self(v0, v1)
    }
}

// Use Rabin-Karp hashing algorithm
fn solve(n: usize, source: &Vec<u8>, target: &Vec<u8>) -> isize {
    let mut t = Hash::zero();
    for &b in target.iter() {
        t = (t << 1) + Hash::new(b as u64);
    }

    let mut s = vec![Hash::zero(); n];
    for (i, &b) in source.iter().enumerate() {
        if i == 0 {
            s[i] = Hash::new(b as u64);
        } else {
            s[i] = (s[i - 1] << 1) + Hash::new(b as u64);
        }
    }

    let mut h = Hash::zero();
    for (i, &b) in source.iter().rev().enumerate() {
        h = h + (Hash::new(b as u64) << i);
        let w = n - i - 1;
        let mut x = h << w;
        if w > 0 { x = x + s[w - 1]; }
        if x == t { return ((i + 1) % n) as isize; }
    }

    -1
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n = io.ln::<usize>();
    let source = io.bytes(n); io.ln::<String>();
    let target = io.bytes(n); io.ln::<String>();
    let ans = solve(n, &source, &target);
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
        self.r.read_exact(&mut buf).expect("Unable to read bytes");
        buf
    }

    fn byte(&mut self) -> u8 { self.bytes(1)[0] }
    fn chr(&mut self) -> char { self.byte() as char }

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
