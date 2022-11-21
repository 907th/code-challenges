fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split(' ');
    let a: f64 = iter.next().unwrap().parse().unwrap();
    let r: f64 = iter.next().unwrap().parse().unwrap();
    const PI: f64 = std::f64::consts::PI;
    let d: f64 = 2_f64.sqrt() * a / 2.0;
    let ans: f64 =
        if d <= r {
            a * a
        } else if 2.0 * r <= a {
            PI * r * r
        } else {
            let alpha: f64 = (a / 2.0 / r).acos();
            let sq_tri: f64 = a * r * alpha.sin() / 2.0;
            let sq_sec: f64 = alpha * r * r;
            PI * r * r - 4.0 * (sq_sec - sq_tri)
        };
    println!("{:.3}", ans);
}
