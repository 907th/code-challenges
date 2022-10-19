const N: usize = 127;

struct Counter {
    target: [usize; N],
    actual: [usize; N],
    matches: usize
}

impl Counter {
    fn new(bytes: &[u8]) -> Self {
        let mut ret = Self{
            target: [0; N],
            actual: [0; N],
            matches: 0
        };
        for b in bytes {
            let i = *b as usize;
            ret.target[i] += 1;
        }
        for i in 0..N {
            if ret.target[i] == 0 { ret.matches += 1; }
        }
        ret
    }

    fn ok(&self) -> bool {
        self.matches == N
    }

    fn add(&mut self, b: u8) {
        let i = b as usize;
        self.actual[i] += 1;
        if self.actual[i] == self.target[i] {
            self.matches += 1;
        }
    }

    fn delete(&mut self, b: u8) {
        let i = b as usize;
        if self.actual[i] == self.target[i] {
            self.matches -= 1;
        }
        self.actual[i] -= 1;
    }
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let (n1, n2) = (b1.len(), b2.len());
        if n1 > n2 { return false; }
        let mut counter = Counter::new(b1);
        for i in 0..n1 { counter.add(b2[i]); }
        if counter.ok() { return true; }
        for i in n1..n2 {
            counter.add(b2[i]);
            counter.delete(b2[i - n1]);
            if counter.ok() { return true; }
        }
        false
    }
}

