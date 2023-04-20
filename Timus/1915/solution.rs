#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::{time, mem};

struct Stack {
    capacity: usize,
    max_length: usize,
    length: usize,
    left: usize,
    right: usize,
    data: Vec<i32>
}

impl Stack {
    fn new(capacity: usize, max_length: usize) -> Self {
        assert!(capacity >= max_length);
        Self {
            capacity,
            max_length,
            length: 0,
            left: 0,
            right: 0,
            data: vec![0; capacity]
        }
    }

    fn push(&mut self, x: i32) {
        self.data[self.right] = x;
        self.right = (self.right + 1) % self.capacity;
        self.length += 1;
        if self.length > self.max_length {
            self.left = (self.left + 1) % self.capacity;
            self.length -= 1;
        }
    }

    fn pop(&mut self) -> i32 {
        assert!(self.length > 0 && self.max_length > 0);
        self.right = (self.right + self.capacity - 1) % self.capacity;
        self.length -= 1;
        self.max_length -= 1; // shorten stack max length too
        self.data[self.right]
    }

    fn duplicate(&mut self) {
        let mut l = (self.left + self.capacity - 1) % self.capacity;
        let mut r = (self.right + self.capacity - 1) % self.capacity;
        let k = min(self.max_length - self.length, self.length);
        for _ in 0..k {
            self.data[l] = self.data[r];
            l = (l + self.capacity - 1) % self.capacity;
            r = (r + self.capacity - 1) % self.capacity;
        }
        self.left = (l + 1) % self.capacity;
        self.length += k;
    }

    #[allow(dead_code)]
    fn debug(&self) {
        println!(
            "cap = {}, max_len = {}, len = {}, left = {}, right = {}\n{:?}",
            self.capacity, self.max_length, self.length, self.left, self.right, self.data
        )
    }
}

fn solve(_n: usize, data: Vec<i32>) -> Vec<i32> {
    let pops_count = data.iter().filter(|&x| *x == -1).count();
    let mut stack = Stack::new(max(pops_count, 1), pops_count); // stack can not have zero capacity
    let mut ans: Vec<i32> = Vec::new();
    // stack.debug();
    for &x in data.iter() {
        match x {
            -1 => ans.push(stack.pop()),
            0 => stack.duplicate(),
            _ => stack.push(x)
        }
        // stack.debug();
    }
    ans
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n: usize = io.ln()?;
    let mut data: Vec<i32> = Vec::new();
    for _ in 0..n {
        let x: i32 = io.ln()?;
        data.push(x);
    }
    let ans = solve(n, data);
    for x in ans { writeln!(io, "{}", x)?; }
    Ok(())
}

fn main() -> IOResult<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut io = IO::new(stdin.lock(), stdout.lock());
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

#[allow(dead_code)]
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

// Benchmark

struct Benchmark<'a> {
    name: &'a str,
    running: bool,
    timer: time::Instant,
    elapsed: u128,
    total: u128
}

#[allow(dead_code)]
impl<'a> Benchmark<'a> {
    fn new(name: &'a str, running: bool) -> Self {
        Self {
            name,
            running,
            timer: time::Instant::now(),
            elapsed: 0,
            total: 0
        }
    }

    fn start(&mut self) {
        assert!(!self.running);
        self.running = true;
        self.timer = time::Instant::now();
    }

    fn stop(&mut self) -> u128 {
        assert!(self.running);
        let elapsed = self.timer.elapsed().as_millis();
        self.running = false;
        self.elapsed = elapsed;
        self.total += elapsed;
        elapsed
    }

    fn debug(&self) {
        println!(
            "{}: elapsed {}ms, total {}ms",
            self.name, self.elapsed, self.total
        );
    }
}

#[allow(dead_code)]
fn sleep(ms: u64) {
    std::thread::sleep(time::Duration::from_millis(ms));
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
