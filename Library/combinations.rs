// Algorithm for combination generation in lexicographic order by Charles J. Mifsud
//
// Time: ?
// Memory: O(1)
// Reference: Charles J. Mifsud. 1963. Algorithm 154: combination in lexicographical order. Commun. ACM 6, 3 (March 1963), 103. https://doi.org/10.1145/366274.366309

// The lexicographically next distinct combination of integers 0, 1, ..., n-1 taken r at a time
// is generated in v unless v already contains maximal combination. In the later case the function
// returns false, otherwise it returns true.
fn next_combination(n: usize, r: usize, v: &mut Vec<usize>) -> bool {
    if v.is_empty() {
        for i in 0..r {
            v.push(i);
        }
        return true;
    }
    if v[r - 1] < n - 1 {
        v[r - 1] = v[r - 1] + 1;
        return true;
    }
    for j in (0..(r - 1)).rev() {
        if v[j] < n - r + j {
            v[j] = v[j] + 1;
            for s in (j + 1)..r {
                v[s] = v[j] + s - j;
            }
            return true;
        }
    }
    false
}

fn main() {
    let mut v: Vec<usize> = Vec::new();
    while next_combination(5, 3, &mut v) {
        println!("{:?}", v);
    }
}
