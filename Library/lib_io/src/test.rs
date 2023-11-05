use super::*;

#[test]
fn test_bytes() {
    let r = String::from("Hello!\n");
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    let first_four_bytes = io.bytes(4).expect("Read 4 bytes");
    let next_three_bytes = io.bytes(3).expect("Read 4 bytes");
    assert!(
        first_four_bytes == "Hell".as_bytes(),
        "First 4 bytes expected"
    );
    assert!(
        next_three_bytes == "o!\n".as_bytes(),
        "Next 3 bytes expected"
    );
}

#[test]
fn test_byte() {
    let r = String::from("Hello!");
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    let first_byte = io.byte().expect("Read first byte");
    let second_byte = io.byte().expect("Read second byte");
    assert!(first_byte == b'H', "First byte expected");
    assert!(second_byte == b'e', "Second byte expected");
}

#[test]
fn test_eof() {
    let r = String::from("Hello,\nworld!\n");
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    let whole_input = io.eof().expect("Read whole input");
    assert!(whole_input == "Hello,\nworld!\n", "Whole input expected");
}

#[test]
fn test_ln() {
    let r = String::from("7\nworld\nA\n");
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    let num: i32 = io.ln().expect("Read first line as number");
    let line: String = io.ln().expect("Read second line as string");
    let chr: char = io.ln().expect("Read third line as char");
    assert!(num == 7, "Number expected");
    assert!(line == "world", "String expected");
    assert!(chr == 'A', "Char expected");
}

#[test]
fn test_sp() {
    let r = String::from("7 world A");
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    let num: i32 = io.sp().expect("Read first word as number");
    let line: String = io.sp().expect("Read second word as string");
    let chr: char = io.sp().expect("Read third word as char");
    assert!(num == 7, "Number expected");
    assert!(line == "world", "String expected");
    assert!(chr == 'A', "Char expected");
}

#[test]
fn test_split_sp() {
    let str_of_num = "1 5 6";
    let str_of_chr = "T 4 z";
    let vec_of_num: Vec<i32> = split_sp(str_of_num).expect("String of numbers");
    let vec_of_chr: Vec<char> = split_sp(str_of_chr).expect("String of chars");
    assert!(
        vec_of_num == vec![1i32, 5i32, 6i32],
        "Vector of numbers expected"
    );
    assert!(
        vec_of_chr == vec!['T', '4', 'z'],
        "Vector of chars expected"
    );
}

#[test]
fn test_writer() {
    let r = String::new();
    let w = Vec::new();
    let mut io = IO::new(r.as_bytes(), w);
    write!(io, "Hello, {}!", "world").expect("Write to IO");
    let w = io.w.into_inner().expect("Extract writer object");
    let w_as_str = String::from_utf8(w).expect("Output as string");
    assert!(w_as_str == "Hello, world!", "String expected");
}
