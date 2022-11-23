struct DP {
    c: [[usize; 11]; 11],
    d: [usize; 11],
}

impl DP {
    fn new() -> Self {
        DP {
            c: [[0; 11]; 11],
            d: [0; 11],
        }
    }

    fn c(&mut self, i: usize, n: usize) -> usize {
        if i == 0 || i == n { return 1; }
        if self.c[i][n] != 0 { return self.c[i][n]; }
        self.c[i][n] = self.c(i, n - 1) + self.c(i - 1, n - 1);
        self.c[i][n]
    }

    fn d(&mut self, n: usize) -> usize {
        if n == 0 { return 1; }
        if self.d[n] != 0 { return self.d[n]; }
        for i in 1..=n {
            self.d[n] += self.c(i, n) * self.d(n - i);
        }
        self.d[n]
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut dp = DP::new();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let n: i32 = buf.trim().parse().unwrap();
        if n == -1 { break; }
        println!("{}", dp.d(n as usize));
    }
}
