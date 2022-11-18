// Solution: Array is split into two parts, each player gets all the cells in one of theese parts.
fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let nums = buf.trim().split(' ').map(|s| s.parse().unwrap());
    let v: Vec<i32> = nums.collect();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut nums = buf.trim().split(' ').map(|s| s.parse::<usize>().unwrap());
    let p1: usize = nums.next().unwrap() - 1;
    let p2: usize = nums.next().unwrap() - 1;

    if p1 == p2 {
        let s1 :i32 = v[0..p1].iter().sum();
        let s2 :i32 = v[(p1 + 1)..n].iter().sum();
        println!("{} {}", std::cmp::max(s1, s2) + v[p1], std::cmp::min(s1, s2));
    } else if p1 < p2 {
        let k = (p2 - p1) / 2;
        let s1 :i32 = v[0..=(p1 + k)].iter().sum();
        let s2 :i32 = v[(p1 + k + 1)..n].iter().sum();
        println!("{} {}", s1, s2);
    } else { // p1 > p2
        let k = (p1 - p2) / 2;
        let s1 :i32 = v[(p1 - k)..n].iter().sum();
        let s2 :i32 = v[0..(p1 - k)].iter().sum();
        println!("{} {}", s1, s2);
    }
}
