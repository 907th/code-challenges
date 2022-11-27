// Data structure for storing disjoint sets.
//
// Time: O(M*A(N)) where M is the number of operations (union of find),
// N is the number of elements, A is the inverse Ackerman function.
// Hint: Having A <= 4 for any practical number, the time complexity is almost linear on M.
// Memory: O(N) where N is the number of elements.
// Reference: https://en.wikipedia.org/wiki/Disjoint-set_data_structure

use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self{
            parent: (0..n).collect(), // Every set is a tree
            rank: vec![0; n] // Estimated tree height
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (i, j) = (self.find(i), self.find(j));
        let (ri, rj) = (self.rank[i], self.rank[j]);
        if i == j { return false; }
        match ri.cmp(&rj) {
            Ordering::Greater => {
                self.parent[j] = i;
            },
            Ordering::Less => {
                self.parent[i] = j;
            }
            Ordering::Equal => {
                self.parent[j] = i;
                self.rank[i] += 1;
            }
        }
        true
    }

    // Tarjan's path splitting algo
    fn find(&mut self, mut i: usize) -> usize {
        let mut pi = self.parent[i];
        while i != pi {
            let gpi = self.parent[pi];
            self.parent[i] = gpi;
            i = pi;
            pi = gpi;
        }
        i
    }
}
