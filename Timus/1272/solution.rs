use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut reader = std::io::BufReader::new(stdin);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let nums: Vec<usize> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let (n, k, _m) = (nums[0], nums[1], nums[2]);
    let mut uf = UnionFind::new(n);
    let mut ans = n - 1;
    for _ in 0..k {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let nums: Vec<usize> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        let (i, j) = (nums[0] - 1, nums[1] - 1);
        if uf.union(i, j) { ans -= 1; }
    }
    println!("{}", ans);
}

// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
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
            std::cmp::Ordering::Greater => {
                self.parent[j] = i;
            },
            std::cmp::Ordering::Less => {
                self.parent[i] = j;
            }
            std::cmp::Ordering::Equal => {
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
