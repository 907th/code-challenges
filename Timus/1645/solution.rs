#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

struct BinaryIndexedTree {
    data: Vec<usize>
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self { data: vec![0; n + 1] }
    }

    // Least significant bit
    fn lsb(i: usize) -> usize {
        i & ((usize::MAX ^ i) + 1)
    }

    // Sum on interval [1, i]
    fn prefix_sum(&self, mut i: usize) -> usize {
        assert!(i > 0, "array indices are 1-based");
        let mut s = 0;
        while i > 0 {
            s += self.data[i];
            i -= Self::lsb(i);
        }
        s
    }

    // Sum on interval [l, r]
    fn range_sum(&self, l: usize, r: usize) -> usize {
        let mut s = self.prefix_sum(r);
        if l > 1 { s -= self.prefix_sum(l - 1); }
        s
    }

    // Add 'add_value' to cell [i]
    fn add(&mut self, mut i: usize, add_value: usize) {
        assert!(i > 0, "array indices are 1-based");
        let n = self.data.len();
        while i < n {
            self.data[i] += add_value;
            i += Self::lsb(i);
        }
    }
}

// For every racer calculate the number of racers who has greater number and finished earlier.
fn solve_min(n: usize, p: &Vec<usize>) -> Vec<usize> {
    let mut bit = BinaryIndexedTree::new(n);
    let mut ans = vec![0usize; n];
    for i in 0..n {
        let x = p[i];
        if x == n {
            ans[x - 1] = 1;
        } else {
            ans[x - 1] = 1 + bit.range_sum(x + 1, n);
        }
        bit.add(x, 1);
    }
    ans
}

// For every racer calculate the number of racers who has lesser number and finished later.
fn solve_max(n: usize, p: &Vec<usize>) -> Vec<usize> {
    let mut bit = BinaryIndexedTree::new(n);
    let mut ans = vec![0usize; n];
    for i in (0..n).rev() {
        let x = p[i];
        if x == 1 {
            ans[x - 1] = n;
        } else {
            ans[x - 1] = n - bit.range_sum(1, x - 1);
        }
        bit.add(x, 1);
    }
    ans
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n: usize = io.ln()?;
    let p: Vec<usize> = split_sp(&io.ln::<String>()?)?;
    let min_pos = solve_min(n, &p);
    let max_pos = solve_max(n, &p);
    for i in 0..n {
        writeln!(io, "{} {}", min_pos[i], max_pos[i])?;
    }
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
}

fn split_sp<T>(s: &str) -> IOResult<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
    let mut vec = Vec::new();
    for part in s.split_whitespace() {
        vec.push(part.parse()?);
    }
    Ok(vec)
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
