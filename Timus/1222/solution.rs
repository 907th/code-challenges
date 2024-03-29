// In the optimal solution all eagles will have either 2 or 3 heads:
// * No eagle can have 1 head obviously.
// * No eagle can have x >= 4 heads (as 2*(x-2) >= x in this case).
// The optimal solution can not contain more than 2 eagles with 2 heads, as 2*2*2 < 3*3.
// Surpisingly, there is only one way to express any number N >= 2 as sum of 3's and not more than two 2's.

use std::fmt;
use std::ops;

type BigIntType = i32;
const BIG_INT_SIZE: usize = 500;
const BIG_INT_BASE: BigIntType = 10000;

#[derive(Clone, Copy)]
struct BigInt {
    data: [BigIntType; BIG_INT_SIZE]
}

impl BigInt {
    fn zero() -> Self {
        BigInt{ data: [0; BIG_INT_SIZE] }
    }

    fn one() -> Self {
        let mut one = Self::zero();
        one.data[0] = 1;
        one
    }
}

impl ops::Mul<BigIntType> for BigInt {
    type Output = Self;
    fn mul(mut self, rhs: BigIntType) -> Self {
        assert!(rhs < BIG_INT_BASE, "BigInt multiplier is too big!");
        let mut o = 0;
        for i in 0..BIG_INT_SIZE {
            let m = self.data[i] * rhs + o;
            self.data[i] = m % BIG_INT_BASE;
            o = m / BIG_INT_BASE;
        }
        assert!(o == 0, "BigInt multiplication overflow!");
        self
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = BIG_INT_SIZE - 1;
        while i > 0 && self.data[i] == 0 { i -= 1; }
        write!(f, "{}", self.data[i])?;
        while i > 0 {
            i -= 1;
            write!(f, "{:0>4}", self.data[i])?;
        }
        Ok(())
    }
}

fn solve(mut n: i32) -> BigInt {
    if n == 1 { return BigInt::one(); }
    let twos = match n % 3 {
        0 => 0,
        1 => 2,
        2 => 1,
        _ => panic!("n % 3 has unexpected value!")
    };
    let mut ans: BigInt = BigInt::one();
    for _i in 0..twos {
        ans = ans * 2;
        n -= 2;
    }
    while n > 1 {
        ans = ans * 3;
        n -= 3;
    }
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read input");
    let n: i32 = buf.trim().parse().expect("parse n");
    println!("{}", solve(n));
}

