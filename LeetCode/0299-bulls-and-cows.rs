impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        const X :u32 = u32::MAX;
        let mut secret: Vec<u32> = secret.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut guess: Vec<u32> = guess.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let (mut bulls, mut cows) = (0, 0);
        for (i, n) in guess.iter_mut().enumerate() {
            if *n == secret[i] {
                *n = X;
                secret[i] = X;
                bulls += 1;
            }
        }
        for n in guess.iter_mut().filter(|x| **x != X) {
            match secret.iter_mut().find(|x| **x == *n) {
                Some(j) => {
                    *n = X;
                    *j = X;
                    cows += 1;
                },
                None => ()
            }
        }
        format!("{}A{}B", bulls, cows)
    }
}
