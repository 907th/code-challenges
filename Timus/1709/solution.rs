#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::{self, FromStr};
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::ops::{Mul};

fn solve(n: usize, d: u64, a: u64, v: &mut Vec<Vec<char>>) -> u64 {
    let mut del = 0u64;
    let mut add = 0u64;
    let mut last_color = 0;
    let mut color = vec![-1; n];
    let mut queue = VecDeque::new();
    for u in 0..n {
        if color[u] != -1 { continue; }
        color[u] = last_color;
        queue.push_back(u);
        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            for j in 0..n {
                if v[i][j] == '1' && color[j] == -1 {
                    v[i][j] = 'k';
                    v[j][i] = 'k';
                    color[j] = last_color;
                    queue.push_back(j);
                }
            }
        }
        last_color += 1;
    }
    // Vertex 0 always has color 0!
    for next_color in 1..last_color {
        for j in 0..n {
            if color[j] == next_color {
                add += 1;
                v[0][j] = 'a';
                v[j][0] = 'a';
                break;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if v[i][j] == '1' {
                v[i][j] = 'd';
                v[j][i] = 'd';
                del += 1;
            }
            if v[i][j] == 'k' {
                v[i][j] = '0';
            }
        }
    }
    a * add + d * del
}

#[allow(unused_must_use)]
fn solve_with_io<R: Read>(inp: &mut Input<R>, out: &mut dyn Write) {
    let n: usize = inp.ln();
    let d: u64 = inp.sp();
    let a: u64 = inp.ln();
    let mut v: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let s: String = inp.ln();
        v.push(s.chars().collect());
    }
    let ans = solve(n, d, a, &mut v);
    writeln!(out, "{}", ans).unwrap();
    for i in 0..n {
        let s = v[i].iter().collect::<String>();
        writeln!(out, "{}", s).unwrap();
    }
}

fn main() {
    let mut inp = Input::new(io::stdin());
    let mut out = io::stdout();
    solve_with_io(&mut inp, &mut out);
}

// I/O

trait InputRead<T> {
    fn chr(&mut self) -> char;
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
}

impl<R: Read, T: FromStr> InputRead<T> for Input<R>
where <T as FromStr>::Err: Debug {
    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.0.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }

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
    }
}
