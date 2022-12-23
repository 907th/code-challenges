#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};
use std::mem;

type Graph = Vec<Vec<Edge>>;

struct Edge {
    vertex: usize,
    weight: u32
}

struct Dist {
    d: Vec<u32>
}

impl Dist {
    fn new(g: &Graph) -> Self {
        let n = g.len();
        let mut d = vec![0; n];
        let mut f = vec![false; n];
        let mut s = Vec::new();
        f[0] = true;
        s.push(0);
        while !s.is_empty() {
            let u = s.pop().unwrap();
            for e in g[u].iter() {
                if f[e.vertex] { continue; }
                f[e.vertex] = true;
                d[e.vertex] = d[u] + e.weight;
                s.push(e.vertex);
            }
        }
        Self { d }
    }

    fn get(&self, u: usize) -> u32 {
        self.d[u]
    }
}

trait MaxValue {
    const MAX: Self;
}

impl MaxValue for usize {
    const MAX: usize = usize::MAX;
}

struct RMQ<T> {
    n: usize,
    rmq: Vec<T>
}

impl <T: Ord + Copy + MaxValue> RMQ<T> {
    fn new(v: &Vec<T>) -> Self {
        let n = v.len();
        let mut rmq = v.clone();
        let mut l = 0;
        let mut r = n;
        while (r - l) > 1 {
            let x = (r - l + 1) / 2;
            rmq.resize(r + x, T::MAX);
            for i in l..r {
                let j = r + (i - l) / 2;
                rmq[j] = min(rmq[j], rmq[i]);
            }
            l = r;
            r = r + x;
        }
        RMQ { n, rmq }
    }

    fn get(&self, mut a: usize, mut b: usize) -> T {
        let rmq = &self.rmq;
        let mut l = 0;
        let mut r = self.n;
        let mut ans = T::MAX;
        while a <= b {
            ans = min(ans, rmq[a]);
            ans = min(ans, rmq[b]);
            if a == b { break; }
            a = r + (a - l + 1) / 2;
            b = r + (b - l - 1) / 2;
            let x = (r - l + 1) / 2;
            l = r;
            r = r + x;
        }
        ans
    }
}

struct LCA {
    rmq: RMQ<usize>,
    revids: Vec<usize>,
    first: Vec<usize>
}

impl LCA {
    fn new(g: &Graph) -> Self {
        let n = g.len();
        let mut id = 0;
        let mut ids = vec![usize::MAX; n];
        let mut revids = vec![usize::MAX; n];
        let mut first = vec![usize::MAX; n];
        let mut order = Vec::new();
        let mut f = vec![false; n];
        let mut s = Vec::new();
        f[0] = true;
        s.push((true, 0));
        while !s.is_empty() {
            let (t, u) = s.pop().unwrap();
            if t {
                revids[id] = u;
                ids[u] = id;
                id += 1;
                first[u] = order.len();
                order.push(ids[u]);
                for e in g[u].iter() {
                    if f[e.vertex] { continue; }
                    f[e.vertex] = true;
                    s.push((false, u));
                    s.push((true, e.vertex));
                }
            } else {
                order.push(ids[u]);
            }
        }
        let rmq = RMQ::new(&order);
        Self { rmq, revids, first }
    }

    fn get(&self, u: usize, v: usize) -> usize {
        let a = self.first[u];
        let b = self.first[v];
        let c = self.rmq.get(min(a, b), max(a, b));
        self.revids[c]
    }
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut g: Graph = Vec::new();
    for _ in 0..n { g.push(Vec::new()); }
    for _ in 0..(n - 1) {
        let u: usize = io.sp();
        let v: usize = io.sp();
        let w: u32 = io.ln();
        g[u].push(Edge { vertex: v, weight: w });
        g[v].push(Edge { vertex: u, weight: w });
    }
    let dist = Dist::new(&g);
    let lca = LCA::new(&g);
    let m: usize = io.ln();
    for _ in 0..m {
        let u: usize = io.sp();
        let v: usize = io.ln();
        let p = lca.get(u, v);
        let ans = dist.get(u) + dist.get(v) - 2 * dist.get(p);
        writeln!(io.w, "{}", ans).unwrap();
    }
}

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    solve_with_io(&mut io);
}

// I/O

struct IO<R: Read, W: Write> {
    r: BufReader<R>,
    w: BufWriter<W>
}

#[allow(dead_code)]
impl<R: Read, W: Write> IO<R, W> {
    fn new(r: R, w: W) -> Self {
        Self{ r: BufReader::new(r), w: BufWriter::new(w) }
    }

    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.r.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }

    fn vec<T: FromStr>(&mut self) -> Vec<T> where <T as FromStr>::Err: Debug {
        let mut buf = String::new();
        self.r.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until<T: FromStr>(&mut self, byte: u8) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
    }

    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug { self.until(0xA) }

    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug { self.until(0x20) }
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{IO, solve_with_io};

    #[test]
    fn run_tests() {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(r) => {
                    println!("Running test {}", t);
                    let w: Vec<u8> = Vec::new();
                    let mut io = IO::new(r, w);
                    solve_with_io(&mut io);
                    let out_received_bytes = io.w.into_inner().expect("Unable to unwrap output");
                    let out_received = std::str::from_utf8(&out_received_bytes).expect("Expected out is not utf8");
                    let out_expected_bytes = fs::read(out_path).expect("Unable to read out file");
                    let out_expected = std::str::from_utf8(&out_expected_bytes).expect("Expected out is not utf8");
                    assert_eq!(out_expected, out_received, "Wrong answer");
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No tests were found");
        println!("Total run {} examples", t - 1);
    }
}
