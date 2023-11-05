use super::read_eval_print;
use lib_io::{IOResult, IO};
use std::{fs, str};

#[test]
fn samples() -> IOResult<()> {
    let mut t = 0;
    loop {
        t += 1;
        let in_path = format!("samples/{}.in", t);
        if fs::metadata(&in_path).is_err() {
            break;
        }
        let out_expected_path = format!("samples/{}.out", t);
        let out_received_path = format!("target/test.out");
        {
            println!("Sample {}", t);
            let r = fs::File::open(&in_path)?;
            let w = fs::File::create(&out_received_path)?;
            let io = IO::new(r, w);
            read_eval_print(io)?;
        }
        {
            let out_received_bytes = fs::read(out_received_path)?;
            let out_received = str::from_utf8(&out_received_bytes)?;
            let out_expected_bytes = fs::read(out_expected_path)?;
            let out_expected = str::from_utf8(&out_expected_bytes)?;
            assert!(
                out_expected == out_received,
                "Wrong answer!\nExpected:\n{}\nReceived:\n{}\n",
                out_expected.trim(),
                out_received.trim()
            );
        }
    }
    assert!(t > 1, "No samples were found");
    println!("Total run {} samples", t - 1);
    Ok(())
}
