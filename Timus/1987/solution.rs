#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};

#[derive(Clone, Copy)]
struct Int(i32, i32);

struct IntDetector {
    int_stack: Vec<Int>,
    id_stack: Vec<usize>,
    last_id: usize
}

impl IntDetector {
    fn new() -> Self {
        Self {
            int_stack: Vec::new(),
            id_stack: Vec::new(),
            last_id: 0
        }
    }

    fn push(&mut self, int: Int) {
        while let Some(last_int) = self.int_stack.last() {
            if last_int.1 < int.0 {
                self.int_stack.pop();
                self.id_stack.pop();
            } else { break; }
        }
        self.int_stack.push(int);
        self.id_stack.push(self.last_id);
        self.last_id += 1;
    }

    fn find(&mut self, c: i32) -> isize {
        while let Some(last_int) = self.int_stack.last() {
            if last_int.1 < c {
                self.int_stack.pop();
                self.id_stack.pop();
            } else {
                let id = *self.id_stack.last().unwrap();
                return id as isize + 1;
            }
        }
        -1
    }
}

fn solve(v: &Vec<Int>, q: &Vec<i32>) -> Vec<isize> {
    let mut ans = Vec::new();
    let mut int_detector = IntDetector::new();
    let mut i = 0;
    for &c in q {
        while i < v.len() && v[i].0 <= c {
            int_detector.push(v[i]);
            i += 1;
        }
        let id = int_detector.find(c);
        ans.push(id);
    }
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut v: Vec<Int> = Vec::new();
    for _ in 0..n {
        let a: i32 = io.sp();
        let b: i32 = io.ln();
        v.push(Int(a, b));
    }
    let m: usize = io.ln();
    let mut q: Vec<i32> = Vec::new();
    for _ in 0..m {
        let c: i32 = io.ln();
        q.push(c);
    }
    let ans = solve(&v, &q);
    for id in ans {
        writeln!(io.w, "{}", id).unwrap();
    }
}

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    solve_with_io(&mut io);
}

// I/O

trait IOFn<T> {
    fn vec(&mut self) -> Vec<T>;
    fn until(&mut self, byte: u8) -> T;
    fn ln(&mut self) -> T { self.until(0xA) }
    fn sp(&mut self) -> T { self.until(0x20) }
}

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
}

impl<R: Read, W: Write, T: FromStr> IOFn<T> for IO<R, W> where <T as FromStr>::Err: Debug {
    fn vec(&mut self) -> Vec<T> {
        let mut buf = String::new();
        self.r.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until(&mut self, byte: u8) -> T {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
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
