#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};
use std::mem;

#[derive(PartialEq)]
enum RoadType { Up, Down }

struct Edge {
    road_type: RoadType,
    vertex: usize
}

type Graph = Vec<Vec<Edge>>;

fn solve(n: usize, g: &Graph, a: usize, b: usize, mut road_type: RoadType) -> usize {
    let mut dist = 0;
    let mut visited = vec![false; n];
    let mut visited2 = vec![false; n];
    let mut distance = vec![0usize; n];
    let mut queue = Vec::new();
    let mut queue2 = Vec::new();
    visited[a] = true;
    visited2[a] = true;
    distance[a] = 0;
    queue.push(a);
    while !queue.is_empty() {
        while !queue.is_empty() {
            let v = queue.pop().unwrap();
            for e in g[v].iter() {
                if visited[e.vertex] { continue; }
                if e.road_type == road_type {
                    visited[e.vertex] = true;
                    distance[e.vertex] = dist;
                    queue.push(e.vertex);
                } else if !visited2[e.vertex] {
                    visited2[e.vertex] = true;
                    queue2.push(e.vertex);
                }
            }
        }
        dist += 1;
        for &v in queue2.iter() {
            if visited[v] { continue; }
            visited[v] = true;
            distance[v] = dist;
            queue.push(v);
        }
        queue2.clear();
        road_type = match road_type {
            RoadType::Up => RoadType::Down,
            RoadType::Down => RoadType::Up
        };
    }
    distance[b]
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.sp();
    let m: usize = io.ln();
    let mut g: Graph = Vec::new();
    for _ in 0..n { g.push(Vec::new()); }
    for _ in 0..m {
        let v: usize = io.sp();
        let u: usize = io.ln();
        g[v - 1].push(Edge { road_type: RoadType::Up, vertex: u - 1 });
        g[u - 1].push(Edge { road_type: RoadType::Down, vertex: v - 1 });
    }
    let a: usize = io.sp();
    let b: usize = io.ln();
    let ans1 = solve(n, &g, a - 1, b - 1, RoadType::Up);
    let ans2 = solve(n, &g, a - 1, b - 1, RoadType::Down);
    writeln!(io.w, "{}", min(ans1, ans2)).unwrap();
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
