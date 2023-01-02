// A value of the polynomail rolling hash function.
// It can be used in Rabin-Karp algorithm for example.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hash(u64, u64);

impl Hash {
    const B0: u64 = 10007;
    const B1: u64 = 10009;
    const P0: u64 = 1000000093;
    const P1: u64 = 1000000097;

    fn new(v: u64) -> Self { Self(v, v) }
    fn zero() -> Self { Self::new(0) }
    fn one() -> Self { Self::new(1) }
}

impl Add for Hash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0.checked_add(other.0).unwrap() % Self::P0,
            self.1.checked_add(other.1).unwrap() % Self::P1
        )
    }
}

impl Mul for Hash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0.checked_mul(other.0).unwrap() % Self::P0,
            self.1.checked_mul(other.1).unwrap() % Self::P1
        )
    }
}

impl Shl<usize> for Hash {
    type Output = Self;
    // x << rhs means x * BASE^rhs (implemented with fast power algorithm).
    // Complexity of this operation is O(log(rhs)).
    fn shl(self, rhs: usize) -> Self {
        let mut i = rhs;
        let mut b0 = Self::B0;
        let mut b1 = Self::B1;
        let mut v0 = self.0;
        let mut v1 = self.1;
        while i > 0 {
            if i & 1 == 1 {
                v0 = v0.checked_mul(b0).unwrap() % Self::P0;
                v1 = v1.checked_mul(b1).unwrap() % Self::P1;
            }
            b0 = b0.checked_mul(b0).unwrap() % Self::P0;
            b1 = b1.checked_mul(b1).unwrap() % Self::P1;
            i = i >> 1;
        }
        Self(v0, v1)
    }
}
