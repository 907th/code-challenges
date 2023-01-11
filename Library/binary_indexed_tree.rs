// Binary indexed tree (Fenwick tree) efficiently calculates prefix sums for array of numbers. Indices are 1-based!
//
// Time: almost all operations are O(log(N)) except for the 'find_from_left' which is O(log^2(N))
// Memory: O(N)
// Reference: https://en.wikipedia.org/wiki/Fenwick_tree

struct BinaryIndexedTree {
    data: Vec<i32>
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self { data: vec![0; n + 1] }
    }

    // TODO: Implement  method '::from(v: Vec<i32>)' with O(N) time complexity

    // Least significant bit
    fn lsb(i: usize) -> usize {
        i & ((usize::MAX ^ i) + 1)
    }

    // Sum on interval [1, i]
    fn prefix_sum(&self, mut i: usize) -> i32 {
        assert!(i > 0, "array indices are 1-based");
        let mut s = 0;
        while i > 0 {
            s += self.data[i];
            i -= Self::lsb(i);
        }
        s
    }

    // Sum on interval [l, r]
    fn range_sum(&self, l: usize, r: usize) -> i32 {
        let mut s = self.prefix_sum(r);
        if l > 1 { s -= self.prefix_sum(l - 1); }
        s
    }

    // Value of cell [i]
    fn get(&self, i: usize) -> i32 {
        self.range_sum(i, i)
    }

    // Add 'add_value' to cell [i]
    fn add(&mut self, mut i: usize, add_value: i32) {
        assert!(i > 0, "array indices are 1-based");
        let n = self.data.len();
        while i < n {
            self.data[i] += add_value;
            i += Self::lsb(i);
        }
    }

    // Set cell [i] value to 'new_value'
    fn set(&mut self, i: usize, new_value: i32) {
        let old_value = self.get(i);
        self.add(i, new_value - old_value);
    }

    // Find maximum index [i] such that sum on [1, n] is <= 'target_sum'
    // NOTE: This function works only when all cell values are non-negative!
    fn find_from_right(&self, mut target_sum: i32) -> usize {
        let n = self.data.len();
        let mut i: usize = 0;
        let mut b = 1usize.reverse_bits();
        while b > 0 {
            if i | b < n && self.data[i | b] <= target_sum {
                target_sum -= self.data[i | b];
                i = i | b;
            }
            b = b >> 1;
        }
        i
    }

    // Find smallest index [i] such that sum on [1, n] is >= 'target_sum'
    // NOTE: This function works only when all cell values are non-negative!
    // NOTE: Complexity of this method is O(log^2(N))
    fn find_from_left(&self, target_sum: i32) -> usize {
        let mut l = 1;
        let mut r = self.data.len();
        while l < r {
            let c = (l + r) / 2;
            if self.prefix_sum(c) < target_sum {
                l = c + 1;
            } else {
                r = c;
            }
        }
        l
    }
}

#[cfg(test)]
mod test {
    use super::BinaryIndexedTree;

    #[test]
    fn test() {
        let mut b = BinaryIndexedTree::new(5);

        b.add(1, 3);
        b.add(3, 5);
        b.add(5, 9);

        // [3, 0, 5, 0, 9]

        assert!(b.prefix_sum(1) == 3, "Bad prefix sum!");
        assert!(b.prefix_sum(2) == 3, "Bad prefix sum!");
        assert!(b.prefix_sum(3) == 8, "Bad prefix sum!");
        assert!(b.prefix_sum(4) == 8, "Bad prefix sum!");
        assert!(b.prefix_sum(5) == 17, "Bad prefix sum!");

        b.set(2, 2);
        b.set(3, 3);
        b.set(4, 4);

        // [3, 2, 3, 4, 9]

        assert!(b.prefix_sum(1) == 3, "Bad prefix sum!");
        assert!(b.prefix_sum(2) == 5, "Bad prefix sum!");
        assert!(b.prefix_sum(3) == 8, "Bad prefix sum!");
        assert!(b.prefix_sum(4) == 12, "Bad prefix sum!");
        assert!(b.prefix_sum(5) == 21, "Bad prefix sum!");

        assert!(b.range_sum(2, 3) == 5, "Bad range sum!");
        assert!(b.range_sum(2, 4) == 9, "Bad range sum!");
        assert!(b.range_sum(3, 3) == 3, "Bad range sum!");
        assert!(b.range_sum(2, 5) == 18, "Bad range sum!");
        assert!(b.range_sum(4, 5) == 13, "Bad range sum!");

        assert!(b.get(1) == 3, "Bad cell value!");
        assert!(b.get(2) == 2, "Bad cell value!");
        assert!(b.get(3) == 3, "Bad cell value!");
        assert!(b.get(4) == 4, "Bad cell value!");
        assert!(b.get(5) == 9, "Bad cell value!");

        assert!(b.find(1) == 0, "Wrong cell was found!");
        assert!(b.find(3) == 1, "Wrong cell was found!");
        assert!(b.find(10) == 3, "Wrong cell was found!");
        assert!(b.find(12) == 4, "Wrong cell was found!");
        assert!(b.find(100) == 5, "Wrong cell was found!");
    }
}
