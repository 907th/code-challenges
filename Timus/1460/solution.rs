#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use std::f64::consts::PI;

const EPS: f64 = 1e-12;

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn lt(a: f64, b: f64) -> bool {
    !eq(a, b) && a < b
}

fn le(a: f64, b: f64) -> bool {
    eq(a, b) || a < b
}

fn zero(a: f64) -> bool {
    eq(a, 0.0)
}

fn negative(a: f64) -> bool {
    lt(a, 0.0)
}

fn positive(a: f64) -> bool {
    lt(0.0, a)
}


#[derive(Copy, Clone, Debug)]
struct P {
    x: f64,
    y: f64,
}

impl P {
    fn new(x: f64, y: f64) -> P {
        assert!(x.is_finite(), "P.x must be finite");
        assert!(y.is_finite(), "P.y must be finite");
        P { x, y }
    }

    fn zero() -> P {
        P::new(0.0, 0.0)
    }

    fn len(self) -> f64 {
        self.dist(P::zero())
    }

    fn dist(self, other: P) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn unit(self) -> P {
        self.scale(1.0 / self.len())
    }

    fn scale(self, factor: f64) -> P {
        P::new(self.x * factor, self.y * factor)
    }

    fn resize(self, new_len: f64) -> P {
        self.scale(new_len / self.len())
    }

    fn rotate(self, angle: f64) -> P {
        P::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
        )
    }

    fn eq(self, other: P) -> bool {
        (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS
    }
}

impl Add for P {
    type Output = P;

    fn add(self, other: P) -> P {
        P::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for P {
    type Output = P;

    fn sub(self, other: P) -> P {
        P::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Clone)]
struct Segments(Vec<(P, P)>);

impl Segments {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add_segment(&mut self, a: P, b: P) {
        self.0.push((a, b));
    }

    fn total_length(&self) -> f64 {
        self.0.iter().map(|&(a, b)| a.dist(b)).sum()
    }

    fn print_debug(&self) {
        println!("{} segments", self.0.len());
        for &(a, b) in self.0.iter() {
            println!("{:?} to {:?} length {}", a, b, a.dist(b));
        }
        println!("Total length {}", self.total_length());
    }

    fn relax(&mut self, other: &Segments) {
        let self_len = self.total_length();
        let other_len = other.total_length();
        if zero(self_len) || other_len < self_len {
            *self = other.clone();
        }
    }
}

fn solve(v: Vec<P>) -> Segments {
    let mut ans = Segments::new();

    //
    // 0 additional points
    //

    // Line-like solution (0 -> 1 -> 2 -> 3)
    let mut p: Vec<usize> = Vec::new();
    while next_permutation(4, &mut p) {
        let mut cur = Segments::new();
        for i in 1..4 { cur.add_segment(v[p[i - 1]], v[p[i]]); }
        ans.relax(&cur);
    }

    // Star-like solution (0 -> 1, 0 -> 2, 0 -> 3)
    for i in 0..4 {
        let mut cur = Segments::new();
        for j in 0..4 {
            if i != j { cur.add_segment(v[i], v[j]); }
        }
        ans.relax(&cur);
    }

    //
    // 1 additional point
    //

    const COMB_4_3: [[usize; 4]; 4] = [[0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 3, 1], [1, 2, 3, 0]];
    for p in COMB_4_3 {
        let (a, b, c, d) = (v[p[0]], v[p[1]], v[p[2]], v[p[3]]);
        let tri = solve_for_3(a, b, c);
        for i in [a, b, c] {
            let mut cur = tri.clone();
            cur.add_segment(d, i);
            ans.relax(&cur);
        }
    }

    //
    // 2 additional points
    //

    const COMB_4_2: [[usize; 4]; 3] = [[0, 1, 2, 3], [0, 2, 1, 3], [0, 3, 1, 2]];
    for p in COMB_4_2 {
        let (a, b, c, d) = (v[p[0]], v[p[1]], v[p[2]], v[p[3]]);
        let cur = solve_for_4(a, b, c, d);
        ans.relax(&cur);
    }

    //
    // End of variants
    //

    ans
}

fn solve_for_3(a: P, b: P, c: P) -> Segments {
    let mut ans = Segments::new();
    for (a, b) in [(a, b), (b, a)] {
        let w = a + (b - a).rotate(PI / 3.0);
        if w.eq(c) { continue; }
        else {
            let r = a.dist(b) / 2.0 / (PI / 6.0).cos();
            let o = a + (b - a).rotate(PI / 6.0).resize(r);
            let z = intersect_circle_and_line(w, c, o, r);
            assert!(z.len() > 0);
            for p in z {
                let mut cur = Segments::new();
                cur.add_segment(a, p);
                cur.add_segment(b, p);
                cur.add_segment(c, p);
                ans.relax(&cur);
            }
        }
    }
    ans
}

fn solve_for_4(a: P, b: P, c: P, d: P) -> Segments {
    let mut ans = Segments::new();
    for (a, b) in [(a, b), (b, a)] {
        let w1 = a + (b - a).rotate(PI / 3.0);
        let r1 = a.dist(b) / 2.0 / (PI / 6.0).cos();
        let o1 = a + (b - a).rotate(PI / 6.0).resize(r1);
        for (c, d) in [(c, d), (d, c)] {
            let w2 = c + (d - c).rotate(PI / 3.0);
            let r2 = c.dist(d) / 2.0 / (PI / 6.0).cos();
            let o2 = c + (d - c).rotate(PI / 6.0).resize(r2);
            if w1.eq(w2) { continue; }
            else {
                let z1 = intersect_circle_and_line(w1, w2, o1, r1);
                let z2 = intersect_circle_and_line(w1, w2, o2, r2);
                for &p1 in &z1 {
                    for &p2 in &z2 {
                        // println!("----------------------------------------");
                        // println!("a = {:?} b = {:?}", a, b);
                        // println!("c = {:?} d = {:?}", c, d);
                        // println!("w1 = {:?} o1 = {:?} r1 = {}", w1, o1, r1);
                        // println!("w2 = {:?} o2 = {:?} r2 = {}", w2, o2, r2);
                        // println!("p1 = {:?} p2 = {:?}", p1, p2);
                        let mut cur = Segments::new();
                        cur.add_segment(a, p1);
                        cur.add_segment(b, p1);
                        cur.add_segment(p1, p2);
                        cur.add_segment(c, p2);
                        cur.add_segment(d, p2);
                        ans.relax(&cur);
                    }
                }
            }
        }
    }
    ans
}

fn intersect_circle_and_line(p1: P, p2: P, o: P, r: f64) -> Vec<P> {
    assert!(!p1.eq(p2));
    let (a, b, c) = ((p2.y - p1.y), (p1.x - p2.x), (p1.y * p2.x - p1.x * p2.y));
    if zero(a) {
        let z = solve_quadratic_equation(
            1.0,
            -2.0 * o.x,
            o.x.powi(2) - r.powi(2) + (c / b + o.y).powi(2)
        );
        z.into_iter().map(|x| P::new(x, -c / b)).collect()
    } else {
        let t = b / a;
        let k = c / a + o.x;
        let z = solve_quadratic_equation(
            t.powi(2) + 1.0,
            2.0 * (t * k - o.y),
            k.powi(2) + o.y.powi(2) - r.powi(2)
        );
        z.into_iter().map(|y| P::new((-b / a) * y - (c / a), y)).collect()
    }
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    if zero(a) {
        if zero(b) {
            if zero(c) {
                panic!("This equation has an infinite number of solutions!");
            } else {
                vec![]
            }
        } else {
            vec![-c / b]
        }
    } else {
        let d = b.powi(2) - 4.0 * a * c;
        if zero(d) {
            vec![-b / (2.0 * a)]
        } else if negative(d) {
            vec![]
        } else {
            vec![
                (-b - d.sqrt()) / (2.0 * a),
                (-b + d.sqrt()) / (2.0 * a)
            ]
        }
    }
}

fn solve_with_io<R: Read, W: Write>(io: &mut IO<R, W>) {
    let n: usize = io.ln();
    for i in 0..n {
        if i > 0 { io.ln::<String>(); }
        let mut v: Vec<P> = Vec::new();
        for _ in 0..4 {
            let x: f64 = io.sp();
            let y: f64 = io.ln();
            v.push(P::new(x, y));
        }
        let ans = solve(v);
        // ans.print_debug();
        let ans_len = ans.total_length();
        writeln!(io.w, "{:.4}", ans_len).unwrap();
    }
}

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    solve_with_io(&mut io);
}

// Generate next perumtation (in lexicographic order)
// Source: https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
fn next_permutation(n: usize, v: &mut Vec<usize>) -> bool {
    if v.is_empty() {
        for i in 0..n { v.push(i); }
        return true;
    }
    if n <= 1 { return false; }
    let mut i = v.len() - 2;
    loop {
        if v[i] < v[i + 1] { break; }
        if i == 0 { return false; } else { i -= 1; }
    }
    let mut l = i + 1;
    let mut r = n;
    while l < r {
        let c = (l + r) / 2;
        if v[c] > v[i] { l = c + 1; } else { r = c; }
    }
    v.swap(i, l - 1);
    v[(i + 1)..n].reverse();
    true
}

// I/O

struct IO<R: Read, W: Write> {
    r: BufReader<R>,
    w: BufWriter<W>,
}

#[allow(dead_code)]
impl<R: Read, W: Write> IO<R, W> {
    fn new(r: R, w: W) -> Self {
        Self {
            r: BufReader::new(r),
            w: BufWriter::new(w),
        }
    }

    fn chr(&mut self) -> char {
        let mut buf = [0u8];
        self.r.read_exact(&mut buf).expect("Unable to read exactly 1 byte");
        buf[0] as char
    }

    fn vec<T: FromStr>(&mut self) -> Vec<T> where <T as FromStr>::Err: Debug {
        let mut buf = String::new();
        self.r.read_line(&mut buf).expect("Unable to read line");
        buf.trim().split(' ').map(|s| s.parse::<T>().expect("Unable to parse string")).collect()
    }

    fn until<T: FromStr>(&mut self, byte: u8) -> T where <T as FromStr>::Err: Debug {
        let mut buf = Vec::new();
        self.r.read_until(byte, &mut buf).expect("Unable to read until specified byte");
        let str = String::from_utf8(buf).expect("String is not utf8");
        str.trim().parse().expect("Unable to parse string")
    }

    fn ln<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.until(0xA)
    }

    fn sp<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
        self.until(0x20)
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::{solve_with_io, IO};
    use std::fs::{self, File};

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
                Err(_) => break,
            }
        }
        assert!(t > 1, "No tests were found");
        println!("Total run {} examples", t - 1);
    }
}
