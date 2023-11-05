// TODO: Reimplement using Library's graph and RMQ data structures.

type Graph = Vec<Vec<Edge>>;

struct Edge {
    vertex: usize,
    weight: u32
}

trait MaxValue {
    const MAX: Self;
}

impl MaxValue for usize {
    const MAX: usize = usize::MAX;
}

struct RMQ<T> {
    n: usize,
    rmq: Vec<T>
}

impl <T: Ord + Copy + MaxValue> RMQ<T> {
    fn new(v: &Vec<T>) -> Self {
        let n = v.len();
        let mut rmq = v.clone();
        let mut l = 0;
        let mut r = n;
        while (r - l) > 1 {
            let x = (r - l + 1) / 2;
            rmq.resize(r + x, T::MAX);
            for i in l..r {
                let j = r + (i - l) / 2;
                rmq[j] = min(rmq[j], rmq[i]);
            }
            l = r;
            r = r + x;
        }
        RMQ { n, rmq }
    }

    fn get(&self, mut a: usize, mut b: usize) -> T {
        let rmq = &self.rmq;
        let mut l = 0;
        let mut r = self.n;
        let mut ans = T::MAX;
        while a <= b {
            ans = min(ans, rmq[a]);
            ans = min(ans, rmq[b]);
            if a == b { break; }
            a = r + (a - l + 1) / 2;
            b = r + (b - l - 1) / 2;
            let x = (r - l + 1) / 2;
            l = r;
            r = r + x;
        }
        ans
    }
}

struct LCA {
    rmq: RMQ<usize>,
    revids: Vec<usize>,
    first: Vec<usize>
}

impl LCA {
    fn new(g: &Graph) -> Self {
        let n = g.len();
        let mut id = 0;
        let mut ids = vec![usize::MAX; n];
        let mut revids = vec![usize::MAX; n];
        let mut first = vec![usize::MAX; n];
        let mut order = Vec::new();
        let mut f = vec![false; n];
        let mut s = Vec::new();
        f[0] = true;
        s.push((true, 0));
        while !s.is_empty() {
            let (t, u) = s.pop().unwrap();
            if t {
                revids[id] = u;
                ids[u] = id;
                id += 1;
                first[u] = order.len();
                order.push(ids[u]);
                for e in g[u].iter() {
                    if f[e.vertex] { continue; }
                    f[e.vertex] = true;
                    s.push((false, u));
                    s.push((true, e.vertex));
                }
            } else {
                order.push(ids[u]);
            }
        }
        let rmq = RMQ::new(&order);
        Self { rmq, revids, first }
    }

    fn get(&self, u: usize, v: usize) -> usize {
        let a = self.first[u];
        let b = self.first[v];
        let c = self.rmq.get(min(a, b), max(a, b));
        self.revids[c]
    }
}
