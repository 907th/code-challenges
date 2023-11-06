#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{Read, Write};
use std::ops::{Add, Mul, Neg, Shl, Sub};
use lib_io::{IOResult, IO};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct P {
    r: usize,
    c: usize,
}

impl P {
    fn new(r: usize, c: usize) -> P {
        P { r, c }
    }
}

struct Accum {
    m: usize,
    h: HashSet<P>,
    total: usize,
    singles: usize,
}

impl Accum {
    fn new(m: usize, v: &Vec<P>) -> Accum {
        let mut h = HashSet::new();
        for p in v {
            h.insert(*p);
        }
        Accum {
            m,
            h,
            total: 0,
            singles: 0,
        }
    }

    fn add(&mut self, p: P, len: usize) {
        if len == 0 {
            return;
        }
        if len == 1 {
            let top = if p.r > 1 {
                self.h.contains(&P::new(p.r - 1, p.c))
            } else {
                true
            };
            let bottom = if p.r < self.m {
                self.h.contains(&P::new(p.r + 1, p.c))
            } else {
                true
            };
            if top && bottom {
                self.singles += 1;
            }
        } else {
            self.total += 1;
        }
    }
}

fn horizontal_intervals_count(m: usize, n: usize, mut v: Vec<P>) -> (usize, usize) {
    v.push(P::new(m, n + 1));
    v.sort_by(|a, b| {
        if a.r != b.r {
            a.r.cmp(&b.r)
        } else {
            a.c.cmp(&b.c)
        }
    });
    let mut accum = Accum::new(m, &v);
    let mut last = P::new(1, 0);
    for p in v.iter() {
        if p.r == last.r {
            // interval between last and p
            accum.add(P::new(last.r, last.c + 1), p.c - last.c - 1);
        } else {
            // end of last's row
            accum.add(P::new(last.r, last.c + 1), n - last.c);
            // empty rows between last and p
            for j in 0..(p.r - last.r - 1) {
                accum.add(P::new(j, 1), n);
            }
            // start of p's row
            accum.add(P::new(p.r, 1), p.c - 1);
        }
        last = *p;
    }
    (accum.total, accum.singles)
}

fn solve(m: usize, n: usize, v: Vec<P>) -> usize {
    let mut s = Vec::new();
    for p in v.iter() {
        s.push(P::new(p.c, p.r));
    }
    let (rows_total, rows_singles) = horizontal_intervals_count(m, n, v);
    let (cols_total, cols_singles) = horizontal_intervals_count(n, m, s);
    assert!(rows_singles == cols_singles);
    rows_total + cols_total + rows_singles
}

fn read_eval_print<R: Read, W: Write>(mut io: IO<R, W>) -> IOResult<()> {
    let m: usize = io.sp()?;
    let n: usize = io.sp()?;
    let k: usize = io.ln()?;
    let mut v = Vec::new();
    for _ in 0..k {
        let r: usize = io.sp()?;
        let c: usize = io.ln()?;
        v.push(P::new(r, c));
    }
    let ans = solve(m, n, v);
    writeln!(io, "{}", ans)?;
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