#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::ops::{Neg, Mul, Add, Sub, Shl};
use std::{time, mem};

#[derive(Clone, Copy, PartialEq)]
enum SolutionStatus {
    Accepted,
    Submitted,
    None
}

struct Contest {
    name: String,
    date: String,
    problems_count: usize,
    solution_status: Vec<SolutionStatus>
}

impl Contest {
    fn solution_status_str(&self) -> String {
        let mut ans: String = self.solution_status.iter()
            .map(|status|
                match status {
                    SolutionStatus::Accepted => 'o',
                    SolutionStatus::Submitted => 'x',
                    SolutionStatus::None => '.'
                }
            )
            .collect();
        for _ in (self.problems_count + 1)..=13 { ans.push(' '); }
        ans
    }
}

fn read_contest<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<Contest> {
    let name: String = io.ln()?;
    let date: String = io.ln()?;
    let problems_count: usize = io.sp()?;
    let submissions_count: usize = io.ln()?;
    let mut solution_status = vec![SolutionStatus::None; problems_count];
    for _ in 0..submissions_count {
        let problem_id: usize = (io.byte()? - ('A' as u8)) as usize;
        io.byte()?; // space
        let verdict: String = io.ln()?;
        if verdict == "Accepted" {
            solution_status[problem_id] = SolutionStatus::Accepted;
        } else {
            if solution_status[problem_id] != SolutionStatus::Accepted {
                solution_status[problem_id] = SolutionStatus::Submitted;
            }
        }
    }
    Ok(Contest{ name, date, problems_count, solution_status })
}

fn read_eval_print<R: Read, W: Write>(io: &mut IO<R, W>) -> IOResult<()> {
    writeln!(io, "+------------------------------+--------+-------------+")?;
    writeln!(io, "|Contest name                  |Date    |ABCDEFGHIJKLM|")?;
    writeln!(io, "+------------------------------+--------+-------------+")?;
    let n: usize = io.ln()?;
    for _ in 0..n {
        let contest = read_contest(io)?;
        writeln!(io, "|{:30}|{}|{:13}|", contest.name, contest.date, contest.solution_status_str())?;
        writeln!(io, "+------------------------------+--------+-------------+")?;
    }
    Ok(())
}

fn main() -> IOResult<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut io = IO::new(stdin.lock(), stdout.lock());
    read_eval_print(&mut io)
}

// I/O

type IOResult<T> = Result<T, Box<dyn std::error::Error>>;

struct IO<R: Read, W: Write> {
    r: BufReader<R>,
    w: BufWriter<W>
}

#[allow(dead_code)]
impl<R: Read, W: Write> IO<R, W> {
    fn new(r: R, w: W) -> Self {
        Self{ r: BufReader::new(r), w: BufWriter::new(w) }
    }

    fn bytes(&mut self, n: usize) -> IOResult<Vec<u8>> {
        let mut buf = vec![0u8; n];
        self.r.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn byte(&mut self) -> IOResult<u8> { Ok(self.bytes(1)?[0]) }

    fn eof(&mut self) -> IOResult<String> {
        let mut str = String::new();
        self.r.read_to_string(&mut str)?;
        Ok(str)
    }

    fn parse_until<T>(&mut self, byte: u8, trim: &[char]) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf)?;
        let str = String::from_utf8(buf)?;
        Ok(str.trim_end_matches(trim).parse()?)
    }

    fn ln<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        self.parse_until(0xA, &[0xD as char, 0xA as char]) // Trim both \r and \n on Windows
    }

    fn sp<T>(&mut self) -> IOResult<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
        self.parse_until(0x20, &[0x20 as char])
    }
}

#[allow(dead_code)]
fn split_sp<T>(s: &str) -> IOResult<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::error::Error + 'static {
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

// Benchmark

struct Benchmark<'a> {
    name: &'a str,
    running: bool,
    timer: time::Instant,
    elapsed: u128,
    total: u128
}

#[allow(dead_code)]
impl<'a> Benchmark<'a> {
    fn new(name: &'a str, running: bool) -> Self {
        Self {
            name,
            running,
            timer: time::Instant::now(),
            elapsed: 0,
            total: 0
        }
    }

    fn start(&mut self) {
        assert!(!self.running);
        self.running = true;
        self.timer = time::Instant::now();
    }

    fn stop(&mut self) -> u128 {
        assert!(self.running);
        let elapsed = self.timer.elapsed().as_millis();
        self.running = false;
        self.elapsed = elapsed;
        self.total += elapsed;
        elapsed
    }

    fn debug(&self) {
        println!(
            "{}: elapsed {}ms, total {}ms",
            self.name, self.elapsed, self.total
        );
    }
}

#[allow(dead_code)]
fn sleep(ms: u64) {
    std::thread::sleep(time::Duration::from_millis(ms));
}

// Tests

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use super::{IO, IOResult, read_eval_print};

    #[test]
    fn run_tests() -> IOResult<()> {
        let mut t = 0;
        loop {
            t += 1;
            let in_path = format!("tests/{}.in", t);
            let out_path = format!("tests/{}.out", t);
            match File::open(in_path) {
                Ok(r) => {
                    println!("Sample {}", t);
                    let w: Vec<u8> = Vec::new();
                    let mut io = IO::new(r, w);
                    read_eval_print(&mut io)?;
                    let out_received_bytes = io.w.into_inner()?;
                    let out_received = std::str::from_utf8(&out_received_bytes)?;
                    let out_expected_bytes = fs::read(out_path)?;
                    let out_expected = std::str::from_utf8(&out_expected_bytes)?;
                    assert_eq!(out_expected, out_received, "Wrong answer");
                }
                Err(_) => break
            }
        }
        assert!(t > 1, "No samples were found");
        println!("Total run {} samples", t - 1);
        Ok(())
    }
}