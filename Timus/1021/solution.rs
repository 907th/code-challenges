use std::cmp::Ordering::{Less, Greater, Equal};

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read_line");
    let n1: usize = buf.trim().parse().expect("parse");
    let mut v1 = Vec::new();
    for _ in 0..n1 {
        buf.clear();
        stdin.read_line(&mut buf).expect("read_line");
        let x: i32 = buf.trim().parse().expect("parse");
        v1.push(x);
    }
    buf.clear();
    stdin.read_line(&mut buf).expect("read_line");
    let n2: usize = buf.trim().parse().expect("parse");
    let mut v2 = Vec::new();
    for _ in 0..n2 {
        buf.clear();
        stdin.read_line(&mut buf).expect("read_line");
        let x: i32 = buf.trim().parse().expect("parse");
        v2.push(x);
    }
    let mut i1 = 0usize;
    let mut i2 = 0usize;
    while i1 < v1.len() && i2 < v2.len() {
        let s = v1[i1] + v2[i2];
        match s.cmp(&10000) {
            Less => i1 += 1,
            Greater => i2 += 1,
            Equal => {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
