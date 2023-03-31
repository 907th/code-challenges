#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

fn get_pair(c: char) -> char {
    if c == ']' { return '['; }
    if c == '[' { return ']'; }
    if c == ')' { return '('; }
    if c == '(' { return ')'; }
    panic!("Unexpected char for pairing: {}", c);
}

fn is_closing(c: char) -> bool {
    return c == ')' || c == ']';
}

#[derive(Clone, Copy)]
enum Op {
    Nothing,
    Insert(usize),
    MakePair(usize),
    Prepend
}

struct Solution {
    n: usize,
    ch: Vec<char>,
    dp: Vec<Vec<usize>>,
    op: Vec<Vec<Op>>
}

const NOT_SET: usize = usize::MAX;

impl Solution {
    fn new(s: String) -> Self {
        let n = s.len();
        let ch: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![NOT_SET; n + 1]; n + 1];
        for i in 0..=n { dp[i][i] = 0; }
        let op = vec![vec![Op::Nothing; n + 1]; n + 1];
        Self { n, ch, dp, op }
    }

    fn solve(&mut self) {
        self.solve_for_interval(0, self.n);
    }

    fn solve_for_interval(&mut self, l: usize, r: usize) -> usize {
        assert!(l <= r, "The left pointer must not be greater than the right one!");
        if self.dp[l][r] != NOT_SET { return self.dp[l][r]; }
        let c = self.ch[l];
        if is_closing(c) {
            self.op[l][r] = Op::Prepend;
            self.dp[l][r] = self.solve_for_interval(l + 1, r) + 2;
        } else {
            for k in (l + 1)..=r {
                if k < r && get_pair(c) == self.ch[k] {
                    let x = 2 +
                        self.solve_for_interval(l + 1, k) +
                        self.solve_for_interval(k + 1, r);
                    if self.dp[l][r] > x {
                        self.dp[l][r] = x;
                        self.op[l][r] = Op::MakePair(k);
                    }
                }
                let x = 2 +
                    self.solve_for_interval(l + 1, k) +
                    self.solve_for_interval(k, r);
                if self.dp[l][r] > x {
                    self.dp[l][r] = x;
                    self.op[l][r] = Op::Insert(k);
                }
            }
        }
        assert!(self.dp[l][r] != NOT_SET, "The value must already be obtained here!");
        self.dp[l][r]
    }

    fn reconstruct(&self) -> String {
        self.reconstruct_interval(0, self.n)
    }

    fn reconstruct_interval(&self, l: usize, r: usize) -> String {
        match self.op[l][r] {
            Op::Nothing => {
                self.ch[l..r].iter().collect()
            },
            Op::Insert(k) => {
                let c = self.ch[l];
                c.to_string() +
                    &self.reconstruct_interval(l + 1, k) +
                    &get_pair(c).to_string() +
                    &self.reconstruct_interval(k, r)
            },
            Op::MakePair(k) => {
                let c = self.ch[l];
                let p = self.ch[k];
                c.to_string() +
                    &self.reconstruct_interval(l + 1, k) +
                    &p.to_string() +
                    &self.reconstruct_interval(k + 1, r)
            },
            Op::Prepend => {
                let c = self.ch[l];
                get_pair(c).to_string() +
                    &c.to_string() +
                    &self.reconstruct_interval(l + 1, r)
            }
        }
    }
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let s = io.ln::<String>()?;
    let mut sol = Solution::new(s);
    sol.solve();
    let ans = sol.reconstruct();
    writeln!(io, "{}", ans)?;
    Ok(())
}

fn main() -> IOResult<()> {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    read_eval_print(&mut io)
}

// I/O

type IOResult<T> = Result<T, Box<dyn std::error::Error>>;

struct IO<R: Read, W: Write> {
    r: BufReader<R>,
    w: BufWriter<W>
}

#[allow(dead_code)]
impl<R: Read, W: Write> IO<R, W> {
    fn new(r: R, w: W) -> Self {
        Self{ r: BufReader::new(r), w: BufWriter::new(w) }
    }

    fn bytes(&mut self, n: usize) -> IOResult<Vec<u8>> {
        let mut buf = vec![0u8; n];
        self.r.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn byte(&mut self) -> IOResult<u8> { Ok(self.bytes(1)?[0]) }

    fn eof(&mut self) -> IOResult<String> {
        let mut str = String::new();
        self.r.read_to_string(&mut str)?;
        Ok(str)
    }

    fn parse_until<T>(&mut self, byte: u8, trim: &[char]) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf)?;
        let str = String::from_utf8(buf)?;
        Ok(str.trim_end_matches(trim).parse()?)
    }

    fn ln<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
    }

    fn sp<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        self.parse_until(0x20, &[0x20 as char])
    }

    fn split<T>(s: &str) -> IOResult<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        let mut vec = Vec::new();
        for part in s.split_whitespace() {
            vec.push(part.parse()?);
        }
        Ok(vec)
    }
}

// IO delegates all Write methods to self.w
impl<R: Read, W: Write> Write for IO<R, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.w.write(buf) }
    fn flush(&mut self) -> std::io::Result<()> { self.w.flush() }
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{IO, IOResult, read_eval_print};

    #[test]
    fn run_tests() -> IOResult<()> {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(r) => {
                    println!("Sample {}", t);
                    let w: Vec<u8> = Vec::new();
                    let mut io = IO::new(r, w);
                    read_eval_print(&mut io)?;
                    let out_received_bytes = io.w.into_inner()?;
                    let out_received = std::str::from_utf8(&out_received_bytes)?;
                    let out_expected_bytes = fs::read(out_path)?;
                    let out_expected = std::str::from_utf8(&out_expected_bytes)?;
                    assert_eq!(out_expected, out_received, "Wrong answer");
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No samples were found");
        println!("Total run {} samples", t - 1);
        Ok(())
    }
}
