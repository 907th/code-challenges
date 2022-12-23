#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};
use std::mem;

const EPS: f64 = 1e-12;

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[derive(Debug)]
struct Block {
    x1: usize,
    x2: usize,
    y: usize
}

impl Block {
    fn get_type(&self, u: usize) -> ConnType {
        if self.x1 == u || self.x2 == u { ConnType::Side }
        else if self.y == u { ConnType::Center }
        else { panic!("{} is not connected to this block", u) }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Weight {
    is_set: bool,
    side: f64,
    center: f64
}

impl Weight {
    fn is_set(&self) -> bool {
        self.is_set
    }

    fn set(&mut self, t: ConnType, v: f64) -> bool {
        let (side, center) = match t {
            ConnType::Side => (v, 2.0 * v),
            ConnType::Center => (v / 2.0, v)
        };
        if self.is_set() {
            eq(side, self.side) && eq(center, self.center)
        } else {
            self.is_set = true;
            self.side = side;
            self.center = center;
            true
        }
    }

    fn get(&self, t: ConnType) -> f64 {
        match t {
            ConnType::Side => self.side,
            ConnType::Center => self.center
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ConnType {
    Side,
    Center
}

enum Solution {
    Any,
    No,
    Some(f64)
}

fn solve(n: usize, blocks: Vec<Block>, repka: usize, dedka: usize) -> Solution {
    // println!("n = {}", n);
    // println!("blocks = {:?}", blocks);
    // println!("repka = {}", repka);
    // println!("dedka = {}", dedka);
    let mut w: Vec<Weight> = vec![Weight::default(); n];
    let mut q: VecDeque<usize> = VecDeque::new();
    let t = blocks[repka].get_type(n + 1);
    assert!(w[repka].set(t, 1.0));
    // println!("repka t = {:?}", t);
    // println!("w[repka] = {:?}", w[repka]);
    q.push_back(repka);
    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        // println!("pop i = {}", i);
        let e = [
            (blocks[i].x1, w[i].side),
            (blocks[i].x2, w[i].side),
            (blocks[i].y, w[i].center)
        ];
        for (j, v) in e {
            if j >= n { continue; }
            let already_set = w[j].is_set();
            let t = blocks[j].get_type(i);
            // println!("next j = {}, v = {}, t = {:?}", j, v, t);
            if !w[j].set(t, v) { return Solution::No; }
            // println!("w[{}] = {:?}", j, w[j]);
            if !already_set { q.push_back(j); }
        }
    }
    let t = blocks[dedka].get_type(n + 2);
    // println!("dedka t = {:?}", t);
    if w[dedka].is_set() {
        // println!("w[dedka] = {:?}", w[dedka]);
        let v = w[dedka].get(t);
        Solution::Some(v)
    } else {
        Solution::Any
    }
}

fn get_id(x: i32, n: usize) -> usize {
    match x {
        0 => n,
        -1 => n + 1,
        -2 => n + 2,
        _ => (x - 1) as usize
    }
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    let mut blocks: Vec<Block> = Vec::new();
    let mut repka: usize = usize::MAX;
    let mut dedka: usize = usize::MAX;
    for i in 0..n {
        let data: Vec<i32> = io.vec();
        assert!(data.len() == 4);
        blocks.push(Block {
            x1: get_id(data[1], n),
            x2: get_id(data[2], n),
            y: get_id(data[3], n)
        });
        if data.iter().any(|&x| x == -1) { repka = i; }
        if data.iter().any(|&x| x == -2) { dedka = i; }
    }
    let ans = solve(n, blocks, repka, dedka);
    match ans {
        Solution::Any => writeln!(io.w, "Any"),
        Solution::No => writeln!(io.w, "No solution"),
        Solution::Some(v) => writeln!(io.w, "{:.4}", v)
    }.unwrap();
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
