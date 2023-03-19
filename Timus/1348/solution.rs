#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn point_to_segment_distance(p: Point, s: (Point, Point)) -> Point {
    let a = p.x - s.0.x;
    let b = p.y - s.0.y;
    let c = s.1.x - s.0.x;
    let d = s.1.y - s.0.y;
    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let shift = if len_sq != 0.0 { dot / len_sq } else { -1.0 };
    if shift < 0.0 { s.0 } else if shift > 1.0 { s.1 } else { Point::new(s.0.x + shift * c, s.0.y + shift * d) }
}

fn solve(a: Point, b: Point, c: Point, l: f64) -> (f64, f64) {
    let dist_min = point_to_segment_distance(c, (a, b)).dist(c);
    let dist_max = f64::max(a.dist(c), b.dist(c));
    (f64::max(dist_min - l, 0f64), f64::max(dist_max - l, 0f64))
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let data = split_whitespace::<f64>(&io.eof());
    assert!(data.len() == 7, "Input is expected to contain 7 numbers");
    let a = Point::new(data[0], data[1]);
    let b = Point::new(data[2], data[3]);
    let c = Point::new(data[4], data[5]);
    let l = data[6];
    let (min, max) = solve(a, b, c, l);
    writeln!(io.w, "{:.2}", min).unwrap();
    writeln!(io.w, "{:.2}", max).unwrap();
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

    fn eof(&mut self) -> String {
        let mut str = String::new();
        self.r.read_to_string(&mut str).expect("Unable to read into string until EOF");
        str
    }

    fn parse_until<T: FromStr>(&mut self, byte: u8, trim: &[char]) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("Buffer contains invalid UTF-8 string");
        str.trim_end_matches(trim).parse().expect("Unable to parse string")
    }
    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
    }
    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0x20, &[0x20 as char])
    }
}

#[allow(dead_code)]
fn split_whitespace<T: FromStr>(s: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    s.split_whitespace().map(|p| p.parse().expect("Unable to parse one of the splitted string parts")).collect()
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
