type Graph = Vec<Vec<Edge>>;

struct Edge {
    vertex: usize,
    weight: u32
}

// TODO: Implement BFS and DFS traits
// Example order from LCA:
// fn new(g: &Graph) -> Self {
//     let n = g.len();
//     let mut id = 0;
//     let mut ids = vec![usize::MAX; n];
//     let mut revids = vec![usize::MAX; n];
//     let mut first = vec![usize::MAX; n];
//     let mut order = Vec::new();
//     let mut f = vec![false; n];
//     let mut s = Vec::new();
//     f[0] = true;
//     s.push((true, 0));
//     while !s.is_empty() {
//         let (t, u) = s.pop().unwrap();
//         if t {
//             revids[id] = u;
//             ids[u] = id;
//             id += 1;
//             first[u] = order.len();
//             order.push(ids[u]);
//             for e in g[u].iter() {
//                 if f[e.vertex] { continue; }
//                 f[e.vertex] = true;
//                 s.push((false, u));
//                 s.push((true, e.vertex));
//             }
//         } else {
//             order.push(ids[u]);
//         }
//     }
//     let rmq = RMQ::new(&order);
//     Self { rmq, revids, first }
// }
