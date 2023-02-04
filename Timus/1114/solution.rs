#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

// Big integer numbers implementation

type BigIntData = i32;
const BIG_INT_SIZE: usize = 10;
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
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
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

// Multi-dimensional array implementation

struct MDArray<T, const N: usize> {
    data: Vec<T>,
    dimensions: [usize; N]
}

impl<T, const N: usize> MDArray<T, N> {
    fn new(dimensions: [usize; N]) -> Self where T: Default + Clone {
        let n = dimensions.iter().product();
        let data = vec![T::default(); n];
        Self { data, dimensions }
    }

    fn get(&self, indices: [usize; N]) -> &T {
        let pos = self.get_data_position_by_indices(&indices);
        &self.data[pos]
    }

    fn set(&mut self, indices: [usize; N], value: T) {
        let pos = self.get_data_position_by_indices(&indices);
        self.data[pos] = value;
    }

    fn get_data_position_by_indices(&self, indices: &[usize; N]) -> usize {
        let mut pos: usize = 0;
        let mut shift: usize = 1;
        for i in (0..N).rev() {
            assert!(indices[i] < self.dimensions[i], "MDArray index {} is out of bound", i);
            pos = pos + indices[i] * shift;
            shift = shift * self.dimensions[i];
        }
        pos
    }
}

// Solution

fn solve(n: usize, a: usize, b: usize) -> BigInt {
    let mut dp: MDArray<BigInt, 3> = MDArray::new([n + 1, a + 1, b + 1]);
    for i in 0..=a {
        for j in 0..=b {
            dp.set([0, i, j], BigInt::one());
        }
    }
    for k in 0..n {
        for i in 0..=a {
        for j in 0..=b {
            for di in 0..=a {
            for dj in 0..=b {
                if (i + di > a) || (j + dj > b) { continue; }
                let idx = [k + 1, i + di, j + dj];
                dp.set(idx, *dp.get(idx) + *dp.get([k, i, j]));
            } }
        } }
    }
    *dp.get([n, a, b])
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.sp();
    let a: usize = io.sp();
    let b: usize = io.ln();
    let ans = solve(n, a, b);
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
