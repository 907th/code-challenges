#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::hash::Hash;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

// A value of the polynomail rolling hash function.
// It can be used in Rabin-Karp algorithm for example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PolyHash(u64, u64);

impl PolyHash {
    const B0: u64 = 10007;
    const B1: u64 = 10009;
    const P0: u64 = 1000000093;
    const P1: u64 = 1000000097;

    fn new(v: u64) -> Self { Self(v % Self::P0, v % Self::P1) }
    fn zero() -> Self { Self::new(0) }
    fn one() -> Self { Self::new(1) }
}

impl Default for PolyHash {
    fn default() -> Self {
        Self::zero()
    }
}


impl Add for PolyHash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0.checked_add(other.0).unwrap() % Self::P0,
            self.1.checked_add(other.1).unwrap() % Self::P1
        )
    }
}

impl Sub for PolyHash {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(
            self.0.checked_add(Self::P0 - other.0).unwrap() % Self::P0,
            self.1.checked_add(Self::P1 - other.1).unwrap() % Self::P1
        )
    }
}

impl Mul for PolyHash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0.checked_mul(other.0).unwrap() % Self::P0,
            self.1.checked_mul(other.1).unwrap() % Self::P1
        )
    }
}

impl Shl<usize> for PolyHash {
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

// Data structure which allows to efficiently calculate
// polynomial rolling hash value of every substring of original string.
struct StrPolyHash{
    hashes: Vec<PolyHash>,
    powers: Vec<PolyHash>
}

impl StrPolyHash {
    fn from(s: &String) -> Self {
        let mut hashes: Vec<PolyHash> = Vec::new();
        let mut powers: Vec<PolyHash> = Vec::new();
        let mut h = PolyHash::zero();
        let mut p = PolyHash::one();
        for c in s.chars() {
            h = (h << 1) + PolyHash::new(c as u64);
            p = p << 1;
            hashes.push(h);
            powers.push(p);
        }
        Self{ hashes, powers }
    }

    fn get(&self, pos: usize, len: usize) -> PolyHash {
        let mut h = self.hashes[pos + len - 1];
        if pos > 0 { h = h - self.hashes[pos - 1] * self.powers[len - 1] }
        h
    }
}

fn solve(s: String) -> usize {
    let mut ans = 0usize;
    let n = s.len();
    let q = StrPolyHash::from(&s);
    for len in 1..=n {
        let mut h: HashSet<PolyHash> = HashSet::new();
        for pos in 0..=(n - len) {
            h.insert(q.get(pos, len));
        }
        ans += h.len();
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let s: String = io.ln();
    let ans = solve(s);
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
