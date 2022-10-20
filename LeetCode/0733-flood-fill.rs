impl Solution {
    const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const PLACEHOLDER_COLOR: i32 = 1 << 16;

    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image.clone();
        let (m, n) = (image.len(), image[0].len());
        let (r, c) = (sr as usize, sc as usize);
        let source_color = image[r][c];
        let mut queue = std::collections::VecDeque::from([(r, c)]);
        image[r][c] = Self::PLACEHOLDER_COLOR;
        while let Some(from) = queue.pop_front() {
            for moving in Self::MOVES {
                if let Some((r, c)) = Self::checked_move(from, moving, m, n) {
                    if image[r][c] == source_color {
                        image[r][c] = Self::PLACEHOLDER_COLOR;
                        queue.push_back((r, c));
                    }
                }
            }
        }
        for r in 0..m {
            for c in 0..n {
                if image[r][c] == Self::PLACEHOLDER_COLOR {
                    image[r][c] = color;
                }
            }
        }
        image
    }

    fn checked_move(from: (usize, usize), moving: (isize, isize), m: usize, n: usize) -> Option<(usize, usize)> {
        if from.0 == 0 && moving.0 == -1 { return None; }
        if from.1 == 0 && moving.1 == -1 { return None; }
        if from.0 == m - 1 && moving.0 == 1 { return None; }
        if from.1 == n - 1 && moving.1 == 1 { return None; }
        Some((
            (from.0 as isize + moving.0) as usize,
            (from.1 as isize + moving.1) as usize
        ))
    }
}
