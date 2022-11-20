const N: usize = 60001;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("read_line");
    let n: usize = buf.trim().parse().expect("parse");

    let mut v = vec![i32::MAX; N];
    v[0] = 0;
    for i in 1..N {
        let mut k: isize = 1;
        while i as isize - k * k >= 0 {
            v[i] = std::cmp::min(v[i], v[(i as isize - k * k) as usize] + 1);
            k += 1;
        }
    }

    println!("{}", v[n])
}
