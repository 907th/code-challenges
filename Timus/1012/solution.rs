#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Mul, Add};
use std::mem;

type BigIntData = i32;
const BIG_INT_SIZE: usize = 500;
const BIG_INT_BASE: BigIntData = 10000;

#[derive(Clone, Copy)]
struct BigInt {
    data: [BigIntData; BIG_INT_SIZE]
}

impl BigInt {
    fn zero() -> Self {
        Self { data: [0; BIG_INT_SIZE] }
    }

    fn one() -> Self {
        BigInt::zero() + 1
    }

    fn new(value: BigIntData) -> Self {
        BigInt::one() * value
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Mul<BigIntData> for BigInt {
    type Output = Self;
    fn mul(mut self, rhs: BigIntData) -> Self {
        const ERROR: &str = "BigInt multiplication overflow!";
        let mut o = 0;
        for i in 0..BIG_INT_SIZE {
            let m = self.data[i].checked_mul(rhs).expect(ERROR).checked_add(o).expect(ERROR);
            self.data[i] = m % BIG_INT_BASE;
            o = m / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

impl std::ops::Add<BigInt> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: BigInt) -> Self {
        const ERROR: &str = "BigInt addition overflow!";
        let mut o = 0;
        for i in 0..BIG_INT_SIZE {
            let s = self.data[i].checked_add(rhs.data[i]).expect(ERROR).checked_add(o).expect(ERROR);
            self.data[i] = s % BIG_INT_BASE;
            o = s / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

impl std::ops::Add<BigIntData> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: BigIntData) -> Self {
        const ERROR: &str = "BigInt addition overflow!";
        let mut o = rhs;
        for i in 0..BIG_INT_SIZE {
            if o == 0 { break; }
            let s = self.data[i].checked_add(o).expect(ERROR);
            self.data[i] = s % BIG_INT_BASE;
            o = s / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

// TODO: Impl other operators (/, %, ==, >, etc).

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut i = BIG_INT_SIZE - 1;
        while i > 0 && self.data[i] == 0 { i -= 1; }
        write!(f, "{}", self.data[i])?;
        while i > 0 {
            i -= 1;
            write!(f, "{:0>4}", self.data[i])?;
        }
        Ok(())
    }
}

fn solve(n: usize, k: usize) -> BigInt {
    let mut dp = vec![vec![BigInt::zero(); 2]; n];
    dp[0][0] = BigInt::new(k as i32 - 1);
    for i in 1..n {
        dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) * (k as i32 - 1);
        dp[i][1] = dp[i - 1][0];
    }
    dp[n - 1][0] + dp[n - 1][1]
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let k: usize = io.ln();
    let ans = solve(n, k);
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
