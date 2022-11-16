fn solve(n: usize, _m: usize, v: Vec<(usize, usize)>, p: Vec<usize>) -> bool {
    let mut graph = vec![Vec::new(); n + 1];
    let mut studied = vec![false; n + 1];

    for (s, u) in v {
        graph[u].push(s);
    }
    for i in p {
        studied[i] = true;
        for &j in &graph[i] {
            if !studied[j] { return false; }
        }
    }

    true
}

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let mut nums = buf.trim().split(' ');
    let n = nums.next().unwrap().parse().unwrap();
    let m = nums.next().unwrap().parse().unwrap();

    let mut v = Vec::new();
    for _i in 0..m {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut nums = buf.trim().split(' ');
        let s = nums.next().unwrap().parse().unwrap();
        let u = nums.next().unwrap().parse().unwrap();
        v.push((s, u));
    }

    let mut p = Vec::new();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut nums = buf.trim().split(' ');
    for _i in 0..n {
        let k = nums.next().unwrap().parse().unwrap();
        p.push(k);
    }

    let ans = solve(n, m, v, p);
    if ans { println!("YES"); } else { println!("NO"); }
}

