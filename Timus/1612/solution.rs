use std::io::{self, BufReader, BufRead};
use std::cmp::Ordering::{Equal, Less, Greater};

fn main() {
    let mut tram = 0;
    let mut trolleybus = 0;
    let lines = BufReader::new(io::stdin()).lines();
    for line in lines.map(|l| l.unwrap()) {
        tram += count(&line, "tram");
        trolleybus += count(&line, "trolleybus");
    }
    match tram.cmp(&trolleybus) {
        Greater => println!("Tram driver"),
        Less => println!("Trolleybus driver"),
        Equal => println!("Bus driver")
    }
}

fn count(line: &str, what: &str) -> usize {
    // periods, commas, dashes, colons, and exclamation and question marks
    line.split(&[' ', '.', ',', '-', ':', '!', '?'][..]).filter(|s| s == &what).count()
}
