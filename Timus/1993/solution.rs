#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Shl};
use std::mem;

fn brackets(ans: &mut Vec<char>, s: &[char], open: char, close: char) {
    let p1 = s.iter().position(|&c| c == open).unwrap();
    let p2 = s.iter().position(|&c| c == close).unwrap();
    for i in (p1 + 1)..p2 { ans.push(s[i]); }
}

fn sentence(ans: &mut Vec<char>, s: &[char]) {
    brackets(ans, s, '{', '}');
    ans.push(' ');
    brackets(ans, s, '(', ')');
    ans.push(' ');
    brackets(ans, s, '[', ']');
}

fn conjunction(ans: &mut Vec<char>, s: &[char]) -> usize {
    let mut i: usize = 0;
    while s[i] == ',' || s[i] == ' ' {
        ans.push(s[i]);
        i += 1;
    }
    while s[i] != '[' && s[i] != '{' && s[i] != '(' {
        ans.push(s[i]);
        i += 1;
    }
    i
}

fn capitalize(ans: &mut Vec<char>) {
    ans[0] = ans[0].to_ascii_uppercase();
    for i in 1..ans.len() { ans[i] = ans[i].to_ascii_lowercase(); }
}

fn solve(s: Vec<char>) -> Vec<char> {
    let mut ans: Vec<char> = Vec::new();
    match s.iter().position(|&c| c == ',') {
        Some(comma) => {
            sentence(&mut ans, &s[0..comma]);
            let len = conjunction(&mut ans, &s[comma..]);
            sentence(&mut ans, &s[(comma + len)..]);
        },
        None => {
            sentence(&mut ans, &s[..]);
        }
    }
    capitalize(&mut ans);
    ans
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let s: String = io.ln();
    let ans = solve(s.chars().collect());
    writeln!(io.w, "{}", ans.into_iter().collect::<String>()).unwrap();
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

    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n];
        self.r.read_exact(&mut buf).expect("Unable to read exact number of bytes");
        buf
    }

    fn byte(&mut self) -> u8 { self.bytes(1)[0] }

    fn parse_until<T: FromStr>(&mut self, byte: u8, trim: &[char]) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not valid utf8");
        str.trim_end_matches(trim).parse().expect("Unable to parse string")
    }

    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
    }
    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.parse_until(0x20, &[0x20 as char])
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
