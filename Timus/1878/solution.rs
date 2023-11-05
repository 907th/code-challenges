#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

type M = [[u8; 4]; 4];

fn is_all_good(x: M) -> bool {
    x[0][0] == x[0][1] &&
    x[0][0] == x[1][0] &&
    x[0][0] == x[1][1] &&
    x[0][2] == x[0][3] &&
    x[0][2] == x[1][2] &&
    x[0][2] == x[1][3] &&
    x[2][0] == x[2][1] &&
    x[2][0] == x[3][0] &&
    x[2][0] == x[3][1] &&
    x[2][2] == x[2][3] &&
    x[2][2] == x[3][2] &&
    x[2][2] == x[3][3]
}

fn layer_to_initial_coord(layer: usize) -> (usize, usize) {
    match layer {
        0 => (0, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (1, 1),
        _ => panic!("bad layer")
    }
}

fn rotate_coord(r: usize, c: usize) -> (usize, usize) {
    (3 - c, r)
}

fn rotate_layer(x: &mut M, layer: usize, movements_count: usize) {
    for _ in 0..movements_count {
        let (mut r, mut c) = layer_to_initial_coord(layer);
        let mut color = x[r][c];
        for _ in 0..4 {
            let (r2, c2) = rotate_coord(r, c);
            let color2 = x[r2][c2];
            x[r2][c2] = color;
            r = r2;
            c = c2;
            color = color2;
        }
    }
}

fn solve(m: M) -> usize {
    let mut ans = usize::MAX;
    for movement in 0..256 {
        let mut x = m.clone();
        let mut total_movements_count = 0;
        for layer in 0..4 {
            let movements_count = (movement >> (2 * layer)) & 3;
            rotate_layer(&mut x, layer, movements_count);
            let forward_or_backward_movements_count = min(movements_count, 4 - movements_count);
            total_movements_count += forward_or_backward_movements_count;
        }
        if is_all_good(x) {
            ans = min(ans, total_movements_count);
        }
    }
    ans
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let mut m: M = [[0; 4]; 4];
    for r in 0..4 {
        let line: Vec<u8> = split_sp(&io.ln::<String>()?)?;
        for c in 0..4 { m[r][c] = line[c]; }
    }
    let ans = solve(m);
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