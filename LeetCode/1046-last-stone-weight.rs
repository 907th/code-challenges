impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        if stones.len() == 1 { return stones[0]; }
        loop {
            let mut xi = 0usize;
            for i in 0..stones.len() {
                if stones[i] > stones[xi] { xi = i; }
            }
            let mut yi = if xi == 0 { 1usize } else { 0usize };
            for i in 0..stones.len() {
                if i != xi && stones[i] > stones[yi] { yi = i; }
            }
            if stones[yi] == 0 { return stones[xi]; }
            stones[xi] = stones[xi] - stones[yi];
            stones[yi] = 0;
        }
    }
}
