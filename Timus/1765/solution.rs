#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

enum Divisors {
    One(usize),
    Two(usize, usize)
}

fn get_divisors(mut n: usize) -> Divisors {
    let mut a = 2;
    while n % a != 0 { a += 1; }
    while n > 1 && n % a == 0 { n /= a; }
    if n == 1 { return Divisors::One(a); }
    let mut b = a + 1;
    while n % b != 0 { b += 1; }
    while n > 1 && n % b == 0 { n /= b; }
    assert!(n == 1, "n have more than two divisors");
    Divisors::Two(a, b)
}

fn solve_for_one_divisor(n: usize, a: usize, blades: &mut Vec<Option<bool>>) {
    let k = n / a;
    // println!("a = {}, k = {}", a, k);
    let mut d = vec![false; k];
    for l in 0..n {
        if blades[l].is_none() {
            d[l % k] = true;
        }
    }
    for i in 0..k {
        if d[i] {
            // println!("remove group {}", i);
            for z in 0..a {
                // println!("remove element {}", i + z * k);
                let r = &mut blades[i + z * k];
                if r.is_some() {
                    assert!(*r == Some(true), "blade has beed deleted twice");
                    *r = Some(false);
                }
            }
        }
    }
}

fn solve_for_two_divisors(n: usize, a: usize, b: usize, blades: &mut Vec<Option<bool>>) {
    let k = n / (a * b);
    // println!("a = {}, b = {}, k = {}", a, b, k);
    let mut d1 = vec![false; b * k];
    let mut d2 = vec![false; a * k];
    for l in 0..n {
        if blades[l].is_none() {
            d1[l % (b * k)] = true;
            d2[l % (a * k)] = true;
        }
    }
    let mut k1 = vec![a * b; k];
    let mut k2 = vec![a * b; k];
    for i in 0..k {
        for j in 0..b {
            if d1[i + j * k] { k1[i] -= a; }
        }
        for j in 0..a {
            if d2[i + j * k] { k2[i] -= b; }
        }
    }
    for i in 0..k {
        // println!("choose {} between {} and {}", i, k1[i], k2[i]);
        if k1[i] >= k2[i] {
            // println!("choose d1 for {}", i);
            for j in 0..b {
                if d1[i + j * k] {
                    // println!("remove group {}", i + j * k);
                    for z in 0..a {
                        // println!("remove element {}", i + j * k + z * b * k);
                        let r = &mut blades[i + j * k + z * b * k];
                        if r.is_some() {
                            assert!(*r == Some(true), "blade has beed deleted twice");
                            *r = Some(false);
                        }
                    }
                }
            }
        } else {
            // println!("choose d2 for {}", i);
            for j in 0..a {
                if d2[i + j * k] {
                    // println!("remove group {}", i + j * k);
                    for z in 0..b {
                        // println!("remove element {}", i + j * k + z * a * k);
                        let r = &mut blades[i + j * k + z * a * k];
                        if r.is_some() {
                            assert!(*r == Some(true), "blade has beed deleted twice");
                            *r = Some(false);
                        }
                    }
                }
            }
        }
    }
}

fn solve(n: usize, d: Vec<usize>) -> Option<Vec<usize>> {
    let mut blades = vec![Some(true); n];
    for x in d.iter() { blades[x - 1] = None; }
    match get_divisors(n) {
        Divisors::One(a) => solve_for_one_divisor(n, a, &mut blades),
        Divisors::Two(a, b) => solve_for_two_divisors(n, a, b, &mut blades)
    }
    let left = blades.iter().filter(|&o| *o == Some(true)).count();
    if left > 0 {
        let mut ans: Vec<usize> = Vec::new();
        for i in 0..n {
            if blades[i] == Some(false) { ans.push(i + 1); }
        }
        Some(ans)
    } else {
        None
    }
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n: usize = io.sp()?;
    let k: usize = io.ln()?;
    let d: Vec<usize> = split_sp(&io.ln::<String>()?)?;
    assert!(d.len() == k, "invalid number of deleted blades has been read");
    match solve(n, d) {
        Some(ans) => {
            writeln!(io, "{}", ans.len())?;
            for (i, x) in ans.iter().enumerate() {
                if i > 0 { write!(io, " ")?; }
                write!(io, "{}", x)?;
            }
            writeln!(io)?;
        },
        None => {
            writeln!(io, "-1")?;
        }
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
