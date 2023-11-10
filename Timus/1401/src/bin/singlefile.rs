#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{Read, Write};
use std::ops::{Add, Mul, Neg, Shl, Sub};
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

mod lib_io {
  use std::io::{BufRead, BufReader, BufWriter, Read, Write};
  
  pub type IOResult<T> = Result<T, Box<dyn std::error::Error>>;
  
  pub struct IO<R: Read, W: Write> {
      r: BufReader<R>,
      w: BufWriter<W>
  }
  
  impl<R: Read, W: Write> IO<R, W> {
      pub fn new(r: R, w: W) -> Self {
          Self{ r: BufReader::new(r), w: BufWriter::new(w) }
      }
  
      pub fn bytes(&mut self, n: usize) -> IOResult<Vec<u8>> {
          let mut buf = vec![0u8; n];
          self.r.read_exact(&mut buf)?;
          Ok(buf)
      }
  
      pub fn byte(&mut self) -> IOResult<u8> { Ok(self.bytes(1)?[0]) }
  
      pub fn eof(&mut self) -> IOResult<String> {
          let mut str = String::new();
          self.r.read_to_string(&mut str)?;
          Ok(str)
      }
  
      pub fn ln<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
          self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
      }
  
      pub fn sp<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
          self.parse_until(0x20, &[0x20 as char])
      }
  
      fn parse_until<T>(&mut self, byte: u8, trim: &[char]) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
          let mut buf = Vec::new();
          self.r.read_until(byte, &mut buf)?;
          let str = String::from_utf8(buf)?;
          Ok(str.trim_end_matches(trim).parse()?)
      }
  }
  
  pub fn split_sp<T>(s: &str) -> IOResult<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
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
}