fn main() {
    let mut v = read_nums();
    let (n1, c1) = (v[0], v[1]);
    v = read_nums();
    let (n2, t2, c2) = (v[0], v[1], v[2]);
    v = read_nums();
    let n3 = v[0];

    let mut minutes = 0;
    v = read_nums();
    for _ in 0..v[0] {
        let t = read_time();
        if t > 6 {
            minutes += (t + 59) / 60;
        }
    }

    let ans1 = n1 + minutes * c1;
    let ans2 = n2 + std::cmp::max(0, minutes - t2) * c2;
    let ans3 = n3;

    println!("Basic:     {}", ans1);
    println!("Combined:  {}", ans2);
    println!("Unlimited: {}", ans3);
}

fn read_nums() -> Vec<i32> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().split(' ').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn read_time() -> i32 {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().split(':').map(|x| x.parse::<i32>().unwrap())
        .reduce(|a, x| a * 60 + x).unwrap()
}
