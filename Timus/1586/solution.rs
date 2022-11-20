const N: usize = 10001;
const M: u64 = 1000000009;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read_line");
    let n: usize = buf.trim().parse().expect("parse");

    let mut primes: Vec<usize> = Vec::new();
    for i in 100..1000 {
        if is_prime(i) { primes.push(i); }
    }

    let mut dp = vec![vec![0u64; 100]; N];
    for p in &primes {
        dp[3][*p % 100] += 1;
    }
    for i in 4..=n {
        for p in &primes {
            dp[i][*p % 100] = (dp[i][*p % 100] + dp[i - 1][*p / 10]) % M;
        }
    }

    let ans: u64 = dp[n].iter().sum::<u64>() % M;
    println!("{}", ans)
}

fn is_prime(n: usize) -> bool {
    if n == 1 { return false; }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 1;
    }
    true
}
