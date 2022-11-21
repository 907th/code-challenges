fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split(' ').map(|x| x.parse::<usize>().unwrap());
    let n = iter.next().unwrap();
    let k = iter.next().unwrap();
    let mut v = Vec::new();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split(' ').map(|x| x.parse::<i32>().unwrap());
    for _ in 0..n {
        v.push(iter.next().unwrap());
    }
    let rows = (n + k - 1) / k;
    for i in 0..rows {
        let mut j = i;
        while j < n {
            print!("{:>4}", v[j]);
            j += rows;
        }
        println!();
    }
}
