use std::io;
use std::str;

fn solve(mut s1: str::Chars, mut s2: str::Chars) -> usize {
    let mut ans = 0;
    let (mut p1, mut p2) = (s1.next(), s2.next());
    loop {
        match (p1, p2) {
            (Some(c1), Some(c2)) => {
                if !(c1 == 'L' && (c2 == 'F' || c2 == 'R')) { p1 = s1.next(); }
                if !(c2 == 'L' && (c1 == 'F' || c1 == 'R')) { p2 = s2.next(); }
                ans += 1;
            },
            (Some(_c1), None) => {
                p1 = s1.next();
                ans += 1;
            },
            (None, Some(_c2)) => {
                p2 = s2.next();
                ans += 1;
            },
            (None, None) => break
        }
    }
    ans
}

fn main() {
    let mut l1 = String::new();
    let mut l2 = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut l1).expect("read line 1");
    stdin.read_line(&mut l2).expect("read line 1");
    let ans = solve(
        l1.trim().chars(),
        l2.trim().chars()
    );
    println!("{}", ans);
}

