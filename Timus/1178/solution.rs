fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut v: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut p: Vec<i32> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        p.push((i + 1) as i32);
        v.push(p);
    }
    v.sort_by(|a, b| {
        if a[0] != b[0] { a[0].cmp(&b[0]) } else { a[1].cmp(&b[1]) }
    });
    for i in 0..(n / 2) {
        println!("{} {}", v[2 * i][2], v[2 * i + 1][2]);
    }
}
