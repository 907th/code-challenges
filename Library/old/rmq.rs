use std::cmp::min;

trait MaxValue {
    const MAX: Self;
}

impl MaxValue for i32 {
    const MAX: i32 = i32::MAX;
}

struct RMQ<T> {
    n: usize,
    rmq: Vec<T>
}

impl <T: Ord + Copy + MaxValue> RMQ<T> {
    fn new(v: &Vec<T>) -> Self {
        let n = v.len();
        let mut rmq = v.clone();
        let mut l = 0;
        let mut r = n;
        while (r - l) > 1 {
            let x = (r - l + 1) / 2;
            rmq.resize(r + x, T::MAX);
            for i in l..r {
                let j = r + (i - l) / 2;
                rmq[j] = min(rmq[j], rmq[i]);
            }
            l = r;
            r = r + x;
        }
        RMQ { n, rmq }
    }

    fn get(&self, mut a: usize, mut b: usize) -> T {
        let rmq = &self.rmq;
        let mut l = 0;
        let mut r = self.n;
        let mut ans = T::MAX;
        while a <= b {
            ans = min(ans, rmq[a]);
            ans = min(ans, rmq[b]);
            if a == b { break; }
            a = r + (a - l + 1) / 2;
            b = r + (b - l - 1) / 2;
            let x = (r - l + 1) / 2;
            l = r;
            r = r + x;
        }
        ans
    }
}

// Random number generator (xorshift*)
// Source: https://en.wikipedia.org/wiki/Xorshift#xorshift*
struct Rand {
    x: u64
}

impl Rand {
    fn new() -> Self {
        Self { x: 1 }
    }

    fn u64(&mut self) -> u64 {
        self.x = self.x ^ (self.x >> 12);
        self.x = self.x ^ (self.x << 25);
        self.x = self.x ^ (self.x >> 27);
        self.x.wrapping_mul(0x2545F4914F6CDD1Du64)
    }

    fn u32(&mut self) -> u32 {
        (self.u64() >> 32) as u32
    }

    fn i64(&mut self) -> i64 {
        self.u64() as i64
    }

    fn i32(&mut self) -> i32 {
        self.u32() as i32
    }

    fn usize(&mut self) -> usize {
        if usize::BITS == 64 { self.u64() as usize } else { self.u32() as usize }
    }

    fn isize(&mut self) -> isize {
        if isize::BITS == 64 { self.u64() as isize } else { self.u32() as isize }
    }
}

#[cfg(test)]
mod test {
    use super::{Rand, RMQ};


    fn rand_vec(rand: &mut Rand, n: usize) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        for _ in 0..n { v.push(rand.i32()); }
        v
    }

    #[test]
    fn stress_test_rmq() {
        let mut rand = Rand::new();

        // Test all ranges for small arrays
        for n in (100..=1000).step_by(100) {
            let v = rand_vec(&mut rand, n);
            let rmq = RMQ::new(&v);
            for l in 0..n {
                for r in l..n {
                    let ans = v[l..=r].iter().min().unwrap();
                    assert!(rmq.get(l, r) == *ans);
                }
            }
        }

        // Test random queries for big array
        let n = 1000000;
        let v = rand_vec(&mut rand, n);
        let rmq = RMQ::new(&v);
        for _ in 0..100 {
            let l = rand.usize() % 500000;
            let r = 500000 + rand.usize() % (n - 500000);
            let ans = v[l..=r].iter().min().unwrap();
            assert!(rmq.get(l, r) == *ans);
        }
    }
}
