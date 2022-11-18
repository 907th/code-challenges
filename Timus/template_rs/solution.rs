#[allow(dead_code, unused_imports)]

use std::io::{self, BufReader, Write};
use std::fmt::{Debug, Display};
use std::str::{self, FromStr};
use std::collections::{HashMap, VecDeque};
use std::cmp::{min, max};
use std::ops::{Add, Sub, Mul, Div, Rem};

fn solve() {

}

fn main() {
    let mut inp = Input::new(io::stdin());
    let mut out = io::stdout();
    solve(&mut inp, &mut out);
}

// IO

struct Input(BufReader);

impl Input {
    fn new<T>(src: T) -> Self {
        Self(BufReader::new(src))
    }

    fn read_line(&mut self) -> String {
        let mut buf = Vec::new();
        self.0.read_until(0xA, &mut buf).expect("read line");
        String::from(buf.trim())
    }

    fn read<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.0.read_until(0x20, &mut buf).expect("read line");
        buf.trim().parse().expect("read & parse")
    }
}

