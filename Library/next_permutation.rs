// Generate next perumtation (in lexicographic order)
// Source: https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
fn next_permutation<T: std::cmp::Ord>(v: &mut Vec<T>) -> bool {
    let n = v.len();
    if n <= 1 { return false; }
    let mut i = v.len() - 2;
    loop {
        if v[i] < v[i + 1] { break; }
        if i == 0 { return false; } else { i -= 1; }
    }
    let mut l = i + 1;
    let mut r = n;
    while l < r {
        let c = (l + r) / 2;
        if v[c] > v[i] { l = c + 1; } else { r = c; }
    }
    v.swap(i, l - 1);
    v[(i + 1)..n].reverse();
    true
}

fn main() {
    let mut v = vec![1, 2, 3];
    loop {
        println!("{:?}", v);
        if !next_permutation(&mut v) { break; }
    }
}
