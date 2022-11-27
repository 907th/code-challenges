// Big integer numbers implementation

type BigIntData = i32;
const BIG_INT_SIZE: usize = 500;
const BIG_INT_BASE: BigIntData = 10000;

#[derive(Clone, Copy)]
struct BigInt {
    data: [BigIntData; BIG_INT_SIZE]
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

impl std::ops::Mul<BigIntData> for BigInt {
    type Output = Self;
    fn mul(mut self, rhs: BigIntData) -> Self {
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

// TODO: Impl other operators (+, /, %, etc).

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
