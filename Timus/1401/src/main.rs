#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{Read, Write};
use std::ops::{Add, Mul, Neg, Shl, Sub};

extern crate lib_io;

use lib_io::{IOResult, IO};

type Ans = Vec<Vec<usize>>;

struct Part {
    id: usize,
    kind: usize,
    r: usize,
    c: usize,
}

struct Solution {
    part_id: usize,
    parts: Vec<Part>,
}

impl Solution {
    fn new() -> Solution {
        Solution {
            part_id: 1,
            parts: Vec::new(),
        }
    }

    fn solve(n: usize, x: usize, y: usize) -> Ans {
        let mut sol = Solution::new();
        sol.gen_parts(n, x, y, 0, 0);
        let mut ans = vec![vec![0; n]; n];
        for part in sol.parts.iter() {
            for r in 0..2 {
                for c in 0..2 {
                    if r * 2 + c != part.kind {
                        ans[part.r + r][part.c + c] = part.id
                    }
                }
            }
        }
        ans
    }

    fn gen_parts(&mut self, n: usize, x: usize, y: usize, r: usize, c: usize) {
        assert!(r <= x && x < r + n);
        assert!(c <= y && y < c + n);
        if n == 1 {
            assert!(x == r && y == c);
            return;
        }
        let d = n / 2;
        let p = ((x - r) / d) * 2 + ((y - c) / d);
        assert!(p < 4);
        self.add_part(p, r + d - 1, c + d - 1);
        self.gen_parts(
            d,
            if p != 0 { r + d - 1 } else { x },
            if p != 0 { c + d - 1 } else { y },
            r,
            c,
        );
        self.gen_parts(
            d,
            if p != 1 { r + d - 1 } else { x },
            if p != 1 { c + d } else { y },
            r,
            c + d,
        );
        self.gen_parts(
            d,
            if p != 2 { r + d } else { x },
            if p != 2 { c + d - 1 } else { y },
            r + d,
            c,
        );
        self.gen_parts(
            d,
            if p != 3 { r + d } else { x },
            if p != 3 { c + d } else { y },
            r + d,
            c + d,
        );
    }

    fn add_part(&mut self, kind: usize, r: usize, c: usize) {
        let part = Part {
            id: self.part_id,
            kind,
            r,
            c,
        };
        self.parts.push(part);
        self.part_id += 1;
    }
}

fn read_eval_print<R: Read, W: Write>(mut io: IO<R, W>) -> IOResult<()> {
    let n: usize = io.ln()?;
    let x: usize = io.sp()?;
    let y: usize = io.ln()?;
    let ans = Solution::solve(1 << n, x - 1, y - 1);
    for r in 0..(1 << n) {
        for c in 0..(1 << n) {
            write!(io, "{}", ans[r][c])?;
            if c < (1 << n) - 1 {
                write!(io, " ")?;
            }
        }
        writeln!(io)?;
    }
    Ok(())
}

fn main() -> IOResult<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let io = IO::new(stdin.lock(), stdout.lock());
    read_eval_print(io)
}

#[cfg(test)]
mod test;
