#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{Read, Write};
use std::ops::{Add, Mul, Neg, Shl, Sub};
use lib_io::{IOResult, IO};

fn solve(a: i32, b: i32) -> i32 {
    a + b
}

fn read_eval_print<R: Read, W: Write>(mut io: IO<R, W>) -> IOResult<()> {
    let a: i32 = io.sp()?;
    let b: i32 = io.ln()?;
    let ans = solve(a, b);
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