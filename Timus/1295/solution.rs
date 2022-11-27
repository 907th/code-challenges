fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    println!("{}", calc_mod(n));
}

fn zeroes(mut sum: u64) -> usize {
    let mut ans = 0;
    while sum % 10 == 0 {
        ans += 1;
        sum /= 10;
    }
    ans
}

fn calc_mod(n: usize) -> usize {
    let (a, mut b, mut c, mut d) = (1, 2, 3, 4);
    for _i in 1..n {
        b = (b * 2) % 1000;
        c = (c * 3) % 1000;
        d = (d * 4) % 1000;
    }
    zeroes(a + b + c + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_naive() {
        let (a, mut b, mut c, mut d) = (1u64, 2u64, 3u64, 4u64);
        for i in 1..30 {
            let sum = a + b + c + d;
            let naive_ans = zeroes(sum);
            let mod_ans = calc_mod(i);
            assert_eq!(naive_ans, mod_ans);
            println!("i = {:>2} | sum = {:>18} | naive = {} | mod = {}", i, sum, naive_ans, mod_ans);
            b *= 2;
            c *= 3;
            d *= 4;
        }
    }
}

