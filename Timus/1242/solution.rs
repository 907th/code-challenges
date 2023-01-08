#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

type Graph = Vec<Vec<usize>>;

struct DFS<'a> {
    graph: &'a Graph,
    visited: Vec<bool>
}

impl <'a> DFS<'a> {
    fn new(g: &'a Graph) -> Self {
        DFS {
            graph: g,
            visited: vec![false; g.len()]
        }
    }

    fn dfs(&mut self, v: usize) {
        if self.visited[v] { return; }
        self.visited[v] = true;
        for u in &self.graph[v] { self.dfs(*u); }
    }
}

fn solve(n: usize, children: Graph, parents: Graph, victims: Vec<usize>) -> Vec<usize> {
    let mut can_be_suspect = vec![true; n];
    let mut children_dfs = DFS::new(&children);
    let mut parents_dfs = DFS::new(&parents);
    for v in victims {
        children_dfs.dfs(v);
        parents_dfs.dfs(v);
    }
    for i in 0..n {
        if children_dfs.visited[i] || parents_dfs.visited[i] {
            can_be_suspect[i] = false;
        }
    }
    can_be_suspect.iter().enumerate().filter_map(|(i, b)| if *b { Some(i) } else { None }).collect()
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.line().unwrap().parse().unwrap();
    let mut children: Graph = vec![Vec::new(); n];
    let mut parents: Graph = vec![Vec::new(); n];
    loop {
        let line = io.line().unwrap();
        if line == "BLOOD" { break; }
        let v: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        children[v[0] - 1].push(v[1] - 1);
        parents[v[1] - 1].push(v[0] - 1);
    }
    let mut victims: Vec<usize> = Vec::new();
    while let Some(v) = io.line().map(|s| s.parse::<usize>().unwrap()) {
        victims.push(v - 1);
    }
    let suspects = solve(n, children, parents, victims);
    if suspects.len() == 0 {
        writeln!(io.w, "0").unwrap();
    } else {
        let str: String = suspects.iter().map(|s| (s + 1).to_string()).collect::<Vec<_>>().join(" ");
        writeln!(io.w, "{}", str).unwrap();
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

    fn line(&mut self) -> Option<String> {
        let mut str = String::new();
        match self.r.read_line(&mut str) {
            Ok(n) => if n > 0 { Some(str.trim_end_matches(&[0xD as char, 0xA as char]).to_string()) } else { None },
            Err(_) => None
        }
    }
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
