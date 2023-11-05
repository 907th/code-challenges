#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{Read, Write};
use std::ops::{Add, Mul, Neg, Shl, Sub};

extern crate lib_io;

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

#[cfg(test)]
mod test;
