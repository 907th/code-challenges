#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;
const H: usize = 7;

const ADD: char = '+';
const DEL: char = '-';

fn op(o: char, i: usize, j: usize) -> String {
    let a = ('A' as u8 + i as u8) as char;
    let b = ('A' as u8 + j as u8) as char;
    [a, b, o].iter().collect()
}

fn solve(mut m: Vec<i32>) -> Option<Vec<String>> {
    let mut ans = VecDeque::<String>::new();

    while m[A] > 0 && m[B] > 0 {
        m[A] -= 1; m[B] -= 1;
        ans.push_back(op(DEL, A, B));
    }

    while m[A] > 0 && m[D] > 0 {
        m[A] -= 1; m[D] -= 1;
        ans.push_back(op(DEL, A, D));
    }

    while m[A] > 0 && m[E] > 0 {
        m[A] -= 1; m[E] -= 1;
        ans.push_back(op(DEL, A, E));
    }

    while m[A] > 0 {
        m[A] -= 1; m[B] -= 1;
        ans.push_back(op(DEL, A, B));

        if m[B] < 0 {
            m[B] += 1; m[C] += 1;
            ans.push_front(op(ADD, B, C));
        }
    }

    while m[B] > 0 {
        m[B] -= 1; m[C] -= 1;
        ans.push_back(op(DEL, B, C));

        if m[C] < 0 {
            m[C] += 1; m[G] += 1;
            ans.push_front(op(ADD, C, G));
        }
    }

    while m[D] > 0 {
        m[D] -= 1; m[H] -= 1;
        ans.push_back(op(DEL, D, H));

        if m[H] < 0 {
            m[H] += 1; m[G] += 1;
            ans.push_front(op(ADD, H, G));
        }
    }

    while m[E] > 0 {
        m[E] -= 1; m[F] -= 1;
        ans.push_back(op(DEL, E, F));

        if m[F] < 0 {
            m[F] += 1; m[G] += 1;
            ans.push_front(op(ADD, F, G));
        }
    }

    while m[C] > 0 {
        m[C] -= 1; m[G] -= 1;
        ans.push_back(op(DEL, C, G));
    }

    while m[H] > 0 {
        m[H] -= 1; m[G] -= 1;
        ans.push_back(op(DEL, H, G));
    }

    while m[F] > 0 {
        m[F] -= 1; m[G] -= 1;
        ans.push_back(op(DEL, F, G));
    }

    if m[G] == 0 { Some(ans.into()) } else { None }
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let m: Vec<i32> = split_sp(&io.ln::<String>()?)?;
    let ans = solve(m);
    match ans {
        Some(ary) => { for s in ary { writeln!(io, "{}", s)?; } },
        None => { writeln!(io, "IMPOSSIBLE")?; }
    };
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
