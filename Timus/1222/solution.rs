// The solution is based on the intuition that all eagles
// in the group must have at least 2 and at most 3 heads only.

fn solve(n: i32) -> BigInt {

}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read input");
    let n: i32 = buf.trim().parse().expect("parse n");
    println!("{}", solve(n));
}

