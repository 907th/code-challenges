#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Edge(usize, usize, usize);

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
            .then(self.0.cmp(&other.0))
            .then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Pair = (usize, usize);

const MAX_LEN: usize = 1_000_000;

fn prim(g: &Vec<Vec<Edge>>, max_len: usize) -> Option<Vec<Pair>> {
    let n = g.len();
    let mut ans = Vec::<Pair>::new();
    let mut used = vec![false; n];
    let mut heap = BinaryHeap::<Edge>::new();
    used[0] = true;
    for e in g[0].iter() {
        if e.2 <= max_len { heap.push(*e); }
    }
    while !heap.is_empty() {
        let Edge(u, v, _l) = heap.pop().unwrap();
        if !used[v] {
            ans.push((u, v));
            used[v] = true;
            for e in g[v].iter() {
               if e.2 <= max_len { heap.push(*e); }
            }
        }
    }
    if ans.len() == n - 1 { Some(ans) } else { None }
}

fn solve(g: Vec<Vec<Edge>>) -> (usize, Vec<Pair>) {
    let mut l = 0;
    let mut r = MAX_LEN;
    while l < r {
        let m = (l + r) / 2;
        match prim(&g, m) {
            Some(_) => { r = m; },
            None => { l = m + 1; }
        }
    }
    let edges = prim(&g, l).unwrap();
    (l, edges)
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n = io.sp::<usize>()?;
    let m = io.ln::<usize>()?;
    let mut g = vec![Vec::<Edge>::new(); n];
    for _i in 0..m {
        let u = io.sp::<usize>()?;
        let v = io.sp::<usize>()?;
        let l = io.ln::<usize>()?;
        g[u - 1].push(Edge(u - 1, v - 1, l));
        g[v - 1].push(Edge(v - 1, u - 1, l));
    }
    let (len, edges) = solve(g);
    writeln!(io, "{}", len)?;
    writeln!(io, "{}", edges.len())?;
    for (u, v) in edges {
        writeln!(io, "{} {}", u + 1, v + 1)?;
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
