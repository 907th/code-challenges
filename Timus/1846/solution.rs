#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::{self, FromStr};
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};

fn gcd(a: i32, b: i32) -> i32 {
    if a == -1 || b == -1 {
        let g = max(a, b);
        assert!(g == -1 || g > 0, "GCD can not be zero");
        return g;
    }
    if b == 0 {
        assert!(a != 0, "Only remainder can be zero");
        return a;
    }
    gcd(b, a % b)
}

// Solution: break entire array of operations into blocks of size SQRT(q) (this is called SQRT-decomposition).
// Then calculate GCD in each block separately. Move from left to right and do the following:
//  - For '+ N' operation - add N to the current block and recalculate current block's GCD.
//    Also remember that N has been seen at position i last time (I use HashMap for that).
//  - For '- N' operation - remove N from the block where it has been seen last time
//    and recalculate that block's GCD.
// Overall complexity is Q * SQRT(Q) where Q is the number of operations.
fn solve(q: usize, mut v: Vec<(char, i32)>) -> Vec<i32> {
    let block_size: usize = max((q as f64).sqrt() as usize, 1);
    let mut block_gcd: Vec<i32> = Vec::new();
    let mut x_last_pos: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..q {
        let (c, x) = v[i];
        let cur_block_num = i / block_size;
        if c == '+' {
            x_last_pos.entry(x).and_modify(|p| p.push(i)).or_insert(vec![i]);
        } else if c == '-' {
            let pv = x_last_pos.get_mut(&x).expect("The position vector must exist");
            let k = pv.pop().expect("There must be at least one position");
            v[k] = ('+', -1);

            // Recalculate GCD of the previous block (current block GCD will be recalculated below)
            let k_block_num = k / block_size;
            if k_block_num < cur_block_num {
                let mut bg: i32 = -1;
                for j in (k_block_num * block_size)..((k_block_num + 1) * block_size) {
                    let (c, y) = v[j];
                    if c != '-' { bg = gcd(bg, y); }
                }
                block_gcd[k_block_num] = bg;
            }
        } else {
            panic!("Unexpected operation: {}", c)
        }

        // Calculate GCD of previous blocks
        let mut pg: i32 = -1;
        for j in 0..cur_block_num {
            pg = gcd(pg, block_gcd[j]);
        }

        // Calculate GCD of current block (upto i)
        let mut cg: i32 = -1;
        for j in (cur_block_num * block_size)..=i {
            let (c, y) = v[j];
            if c != '-' { cg = gcd(cg, y); }
        }
        if block_gcd.len() < cur_block_num + 1 {
            block_gcd.push(cg);
        } else {
            block_gcd[cur_block_num] = cg;
        }

        let g = gcd(pg, cg);
        ans.push(if g == -1 { 1 } else { g }); // "GCD of an empty collection is equal to one" (from task description)
    }
    ans
}

fn solve_with_io<R: Read>(inp: &mut Input<R>, out: &mut dyn Write) {
    let q: usize = inp.ln();
    let mut v: Vec<(char, i32)> = Vec::new();
    for _ in 0..q {
        let c: char = inp.chr();
        let x: i32 = inp.ln();
        v.push((c, x));
    }
    let ans = solve(q, v);
    for g in ans {
        writeln!(out, "{}", g).unwrap();
    }
}

fn main() {
    let mut inp = Input::new(io::stdin());
    let mut out = io::stdout();
    solve_with_io(&mut inp, &mut out);
}

// I/O

trait InputRead<T> {
    fn vec(&mut self) -> Vec<T>;
    fn until(&mut self, byte: u8) -> T;
    fn ln(&mut self) -> T { self.until(0xA) }
    fn sp(&mut self) -> T { self.until(0x20) }
}

struct Input<R>(BufReader<R>);

impl<R: Read> Input<R> {
    fn new(r: R) -> Self {
        Self(BufReader::new(r))
    }

    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.0.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }
}

impl<R: Read, T: FromStr> InputRead<T> for Input<R>
where <T as FromStr>::Err: Debug {
    fn vec(&mut self) -> Vec<T> {
        let mut buf = String::new();
        self.0.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until(&mut self, byte: u8) -> T {
        let mut buf = Vec::new();
        self.0.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
    }
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{Input, solve_with_io};

    #[test]
    fn run_tests() {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(in_file) => {
                    let mut inp = Input::new(in_file);
                    let mut out: Vec<u8> = Vec::new();
                    solve_with_io(&mut inp, &mut out);
                    let out_expected = fs::read(out_path).expect("Unable to read out file");
                    assert_eq!(
                        out, out_expected,
                        "Wrong answer:\n{}\nExpected:\n{}\n",
                        std::str::from_utf8(&out).unwrap(),
                        std::str::from_utf8(&out_expected).unwrap()
                    );
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No tests were found");
        println!("Total run {} examples", t - 1);
    }
}
