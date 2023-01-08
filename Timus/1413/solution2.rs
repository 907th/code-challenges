#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

#[derive(Clone, Copy, Debug)]
struct Num {
    a: i64,
    b: f64
}

impl Num {
    fn new(a: i64, b: f64) -> Self {
        Self { a, b }.normalize()
    }

    fn zero() -> Self {
        Self::new(0, 0.0)
    }

    fn one() -> Self {
        Self::new(1, 0.0)
    }

    fn sqrt() -> Self {
        Self::new(0, 0.5f64.sqrt())
    }

    fn normalize(mut self) -> Self {
        if self.b.abs() >= 1.0 {
            let x = self.b as i64;
            self.a = self.a + x;
            self.b = self.b - x as f64;
        }
        assert!(self.b.abs() < 1.0, "Fractional part must be in (-1, 1) interval!");
        if self.a > 0 && self.b < 0.0 {
            self.a = self.a - 1;
            self.b = self.b + 1.0;
        }
        if self.a < 0 && self.b > 0.0 {
            self.a = self.a + 1;
            self.b = self.b - 1.0;
        }
        assert!(self.b.abs() < 1.0, "Fractional part must still be in (-1, 1) interval!");
        assert!((self.a >= 0 && self.b >= 0.0) || (self.a <= 0 && self.b <= 0.0), "Both parts must have the same sign!");
        self
    }

    fn is_negative(&self) -> bool {
        self.a < 0 || self.b < 0.0
    }
}

impl Neg for Num {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.a, -self.b)
    }
}

impl Add<Num> for Num {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.a + other.a, self.b + other.b)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_negative() { write!(f, "-")?; }
        write!(f, "{}.{}", self.a.abs(), (self.b.abs() * 1e16) as i64)
    }
}

fn solve(s: &String) -> (Num, Num) {
    let movements: [(Num, Num); 10] = [
        (  Num::zero(),  Num::zero() ), // 0
        ( -Num::sqrt(), -Num::sqrt() ), // 1
        (  Num::zero(), -Num::one()  ), // 2
        (  Num::sqrt(), -Num::sqrt() ), // 3
        ( -Num::one(),   Num::zero() ), // 4
        (  Num::zero(),  Num::zero() ), // 5
        (  Num::one(),   Num::zero() ), // 6
        ( -Num::sqrt(),  Num::sqrt() ), // 7
        (  Num::zero(),  Num::one()  ), // 8
        (  Num::sqrt(),  Num::sqrt() )  // 9
    ];
    let mut x = Num::zero();
    let mut y = Num::zero();
    for c in s.chars() {
        if c == '0' { break; }
        let (dx, dy) = movements[(c as u8 - '0' as u8) as usize];
        x = x + dx;
        y = y + dy;
    }
    (x, y)
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let s = io.ln::<String>();
    let (x, y) = solve(&s);
    writeln!(io.w, "{} {}", x, y).unwrap();
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
