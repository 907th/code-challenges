#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{self, Debug};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::{self, FromStr};

fn solve(n: usize, v: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..n {
        ans += v[i];
    }
    ans
}

#[allow(unused_must_use)]
fn solve_with_io<R: Read>(inp: &mut Input<R>, out: &mut dyn Write) {
    let n: usize = inp.eol();
    let v: Vec<i32> = inp.veol(n);
    let ans = solve(n, v);
    writeln!(out, "{}", ans).unwrap();
}

fn main() {
    let mut inp = Input::new(io::stdin());
    let mut out = io::stdout();
    solve_with_io(&mut inp, &mut out);
}

// I/O

trait InputRead<T> {
    fn until(&mut self, byte: u8) -> T;

    fn eol(&mut self) -> T {
        self.until(0xA)
    }

    fn sp(&mut self) -> T {
        self.until(0x20)
    }

    fn veol(&mut self, n: usize) -> Vec<T> {
        let mut v = Vec::new();
        for _ in 1..n { v.push(self.sp()); }
        v.push(self.eol());
        v
    }
}

struct Input<R>(BufReader<R>);

impl<R: Read> Input<R> {
    fn new(r: R) -> Self {
        Self(BufReader::new(r))
    }
}

impl<R: Read, T: FromStr> InputRead<T> for Input<R>
where <T as FromStr>::Err: Debug
{
    fn until(&mut self, byte: u8) -> T {
        let mut buf = Vec::new();
        self.0.read_until(byte, &mut buf).expect("Read until byte failed");
        if let Some(&b) = buf.last() {
            if byte == b { buf.pop(); }
        }
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.parse().expect("String parsing failed")
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
