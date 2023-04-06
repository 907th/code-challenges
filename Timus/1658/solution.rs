#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

struct DP {
    last_digit: Vec<Vec<u8>>,
    total_length: Vec<Vec<u8>>
}

impl DP {
    const MAX_LEN: u8 = 100;
    const N1: usize = 9 * Self::MAX_LEN as usize;
    const N2: usize = 81 * Self::MAX_LEN as usize;

    fn new() -> Self {
        let mut last_digit = vec![vec![0u8; Self::N2 + 1]; Self::N1 + 1];
        let mut total_length = vec![vec![Self::MAX_LEN + 1; Self::N2 + 1]; Self::N1 + 1];
        total_length[0][0] = 0;
        for i1 in 0..=Self::N1 {
            for i2 in 0..=Self::N2 {
                if total_length[i1][i2] >= Self::MAX_LEN { continue; }
                // It is important to relax solutions starting from 9 down to 1,
                // because we will reconstruct the original number from top down.
                // See [Self#reconstruct].
                for d in (1u8..=9u8).rev() {
                    let (j1, j2) = (i1 + d as usize, i2 + (d * d) as usize);
                    if j1 > Self::N1 || j2 > Self::N2 { break; }
                    // It is important to compare with >= here also!
                    if total_length[j1][j2] >= total_length[i1][i2] + 1 {
                        total_length[j1][j2] = total_length[i1][i2] + 1;
                        last_digit[j1][j2] = d;
                    }
                }
            }
        }
        Self { last_digit, total_length }
    }

    // I reconstruct the solution from top down. So the DP must be written so that it always
    // find a solution with smallest possible last_digit at the last_digit[a][b] position.
    fn reconstruct(&self, mut a: usize, mut b: usize) -> String {
        if a > Self::N1 || b > Self::N2 || self.total_length[a][b] > Self::MAX_LEN { return "No solution".to_string(); }
        let mut s: Vec<char> = Vec::new();
        while a > 0 && b > 0 {
            let d = self.last_digit[a][b];
            s.push(char::from_digit(d as u32, 10).unwrap());
            a -= d as usize;
            b -= (d * d) as usize;
        }
        for i in 1..s.len() { assert!(s[i - 1] <= s[i]); }
        assert!(a == 0 && b == 0);
        s.iter().collect()
    }
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let dp = DP::new();
    let t: usize = io.ln()?;
    for _ in 0..t {
        let (a, b): (usize, usize) = (io.sp()?, io.ln()?);
        writeln!(io, "{}", dp.reconstruct(a, b))?;
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
