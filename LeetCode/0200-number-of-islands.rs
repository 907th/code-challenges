use std::collections::VecDeque;
use std::cmp::max;

impl Solution {
    const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const LAND: char = '1';
    const MARK: char = '2';

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid.clone();
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans: usize = 0;
        for r in 0..m {
            for c in 0..n {
                if (grid[r][c] == Self::LAND) {
                    Self::flood_fill(&mut grid, r, c, m, n);
                    ans += 1;
                }
            }
        }
        ans as i32
    }

    fn flood_fill(grid: &mut Vec<Vec<char>>, r: usize, c: usize, m: usize, n: usize) {
        let mut queue = VecDeque::from([(r, c)]);
        grid[r][c] = Self::MARK;
        while let Some(from) = queue.pop_front() {
            for moving in Self::MOVES {
                if let Some((r, c)) = Self::checked_move(from, moving, m, n) {
                    if grid[r][c] == Self::LAND {
                        queue.push_back((r, c));
                        grid[r][c] = Self::MARK;
                    }
                }
            }
        }
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

