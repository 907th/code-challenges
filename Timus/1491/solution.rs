#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::{time, mem};

fn solve(n: usize, v: Vec<(usize, usize, u32)>) -> Vec<u32> {
    let mut start: Vec<usize> = (0..=n).into_iter().collect();
    let mut finish: Vec<usize> = (0..=n).into_iter().collect();
    start.sort_by(|&i, &j| v[i].0.cmp(&v[j].0));
    finish.sort_by(|&i, &j| v[i].1.cmp(&v[j].1));
    let mut ans: Vec<u32> = Vec::new();
    let mut s = 0usize;
    let mut f = 0usize;
    let mut x = 0u32;
    for i in 0..n {
        while s < start.len() && v[start[s]].0 <= i + 1 {
            x += v[start[s]].2;
            s += 1;
        }
        while f < finish.len() && v[finish[f]].1 < i + 1 {
            x -= v[finish[f]].2;
            f += 1;
        }
        ans.push(x);
    }
    ans
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n: usize = io.ln()?;
    let mut v: Vec<(usize, usize, u32)> = Vec::new();
    for _ in 0..=n {
        let a: usize = io.sp()?;
        let b: usize = io.sp()?;
        let c: u32 = io.ln()?;
        v.push((a, b, c));
    }
    let ans = solve(n, v);
    for (i, &c) in ans.iter().enumerate() {
        if i > 0 { write!(io, " ")?; }
        write!(io, "{}", c)?;
    }
    writeln!(io)?;
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
