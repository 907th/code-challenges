#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::mem;

#[derive(Copy, Clone)]
struct Edge {
    b: usize,
    c: u32
}

struct Graph {
    n: usize,
    edges: Vec<Vec<Edge>>
}

impl Graph {
    fn new(n: usize) -> Self {
        let edges = vec![Vec::new(); n];
        Graph { n, edges }
    }

    fn add_edge(&mut self, a: usize, b: usize, c: u32) {
        self.edges[a].push(Edge { b, c })
    }
}

const INF: u32 = 100_000_000;

struct DFS<'a> {
    g: &'a Graph,
    v: Vec<bool>,
    d: Vec<u32>
}

impl<'a> DFS<'a> {
    fn new(g: &'a Graph) -> Self {
        let v = vec![false; g.n];
        let d = vec![0; g.n];
        Self { g, v, d }
    }

    fn solve(&mut self, s: usize, f: usize) -> Option<u32> {
        self.v[f] = true;
        self.d[f] = INF;
        let dist = self.dfs(s);
        if dist > INF { Some(dist - INF) } else { None }
    }

    fn dfs(&mut self, a: usize) -> u32 {
        if self.v[a] { return self.d[a]; }
        self.v[a] = true;
        for i in 0..self.g.edges[a].len() {
            let e = self.g.edges[a][i];
            self.d[a] = max(self.d[a], self.dfs(e.b) + e.c);
        }
        self.d[a]
    }
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    let n: usize = io.sp()?;
    let m: usize = io.ln()?;
    let mut g = Graph::new(n);
    for _ in 0..m {
        let a: usize = io.sp()?;
        let b: usize = io.sp()?;
        let c: u32 = io.ln()?;
        g.add_edge(a - 1, b - 1, c);
    }
    let s: usize = io.sp()?;
    let f: usize = io.ln()?;
    let mut dfs = DFS::new(&g);
    let ans = dfs.solve(s - 1, f - 1);
    match ans {
        Some(dist) => writeln!(io, "{}", dist),
        None => writeln!(io, "No solution")
    }?;
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
