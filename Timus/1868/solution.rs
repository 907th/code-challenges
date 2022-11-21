use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    const MEDALS: [&str; 3] = ["gold", "silver", "bronze"];
    let mut places: HashMap<String, String> = HashMap::new();
    for i in 0..12 {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        places.insert(String::from(buf.trim()), String::from(MEDALS[i / 4]));
    }

    let mut matches: usize = 0;
    let mut winners: usize = 0;

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let k: usize = buf.trim().parse().unwrap();
        let mut ok: usize = 0;
        for _ in 0..k {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut iter = buf.trim().split(" : ");
            let predicted_command = iter.next().unwrap();
            let predicted_place = iter.next().unwrap();
            match places.get(predicted_command) {
                Some(place) => if predicted_place == place { ok += 1; },
                None => ()
            }
        }
        if ok > matches {
            matches = ok;
            winners = 1;
        } else if ok == matches {
            winners += 1;
        }
    }

    println!("{}", 5 * winners);
}
