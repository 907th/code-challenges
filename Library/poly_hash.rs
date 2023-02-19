// A value of the polynomail rolling hash function.
// It can be used in Rabin-Karp algorithm for example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PolyHash(u64, u64);

impl PolyHash {
    const B0: u64 = 10007;
    const B1: u64 = 10009;
    const P0: u64 = 1000000093;
    const P1: u64 = 1000000097;

    fn new(v: u64) -> Self { Self(v % Self::P0, v % Self::P1) }
    fn zero() -> Self { Self::new(0) }
    fn one() -> Self { Self::new(1) }
}

impl Default for PolyHash {
    fn default() -> Self {
        Self::zero()
    }
}


impl Add for PolyHash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0.checked_add(other.0).unwrap() % Self::P0,
            self.1.checked_add(other.1).unwrap() % Self::P1
        )
    }
}

impl Sub for PolyHash {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(
            self.0.checked_add(Self::P0 - other.0).unwrap() % Self::P0,
            self.1.checked_add(Self::P1 - other.1).unwrap() % Self::P1
        )
    }
}

impl Mul for PolyHash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0.checked_mul(other.0).unwrap() % Self::P0,
            self.1.checked_mul(other.1).unwrap() % Self::P1
        )
    }
}

impl Shl<usize> for PolyHash {
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

// Data structure which allows to efficiently calculate
// polynomial rolling hash value of every substring of original string.
struct StrPolyHash{
    hashes: Vec<PolyHash>,
    powers: Vec<PolyHash>
}

impl StrPolyHash {
    fn from(s: &String) -> Self {
        let mut hashes: Vec<PolyHash> = Vec::new();
        let mut powers: Vec<PolyHash> = Vec::new();
        let mut h = PolyHash::zero();
        let mut p = PolyHash::one();
        for c in s.chars() {
            h = (h << 1) + PolyHash::new(c as u64);
            p = p << 1;
            hashes.push(h);
            powers.push(p);
        }
        Self{ hashes, powers }
    }

    fn get(&self, pos: usize, len: usize) -> PolyHash {
        let mut h = self.hashes[pos + len - 1];
        if pos > 0 { h = h - self.hashes[pos - 1] * self.powers[len - 1] }
        h
    }
}
