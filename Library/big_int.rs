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
        Self { data: [0; BIG_INT_SIZE] }
    }

    fn one() -> Self {
        BigInt::zero() + 1
    }

    fn new(value: BigIntData) -> Self {
        BigInt::one() * value
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Mul<BigIntData> for BigInt {
    type Output = Self;
    fn mul(mut self, rhs: BigIntData) -> Self {
        const ERROR: &str = "BigInt multiplication overflow!";
        let mut o = 0;
        for i in 0..BIG_INT_SIZE {
            let m = self.data[i].checked_mul(rhs).expect(ERROR).checked_add(o).expect(ERROR);
            self.data[i] = m % BIG_INT_BASE;
            o = m / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

impl std::ops::Add<BigInt> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: BigInt) -> Self {
        const ERROR: &str = "BigInt addition overflow!";
        let mut o = 0;
        for i in 0..BIG_INT_SIZE {
            let s = self.data[i].checked_add(rhs.data[i]).expect(ERROR).checked_add(o).expect(ERROR);
            self.data[i] = s % BIG_INT_BASE;
            o = s / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

impl std::ops::Add<BigIntData> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: BigIntData) -> Self {
        const ERROR: &str = "BigInt addition overflow!";
        let mut o = rhs;
        for i in 0..BIG_INT_SIZE {
            if o == 0 { break; }
            let s = self.data[i].checked_add(o).expect(ERROR);
            self.data[i] = s % BIG_INT_BASE;
            o = s / BIG_INT_BASE;
        }
        assert!(o == 0, "{}", ERROR);
        self
    }
}

// TODO: Impl other operators (/, %, ==, >, etc).

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
