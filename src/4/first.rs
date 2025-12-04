use std::error::Error;
use std::fs;

const NEIGHBOR_DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn solve(grid: &Vec<Vec<char>>) -> u32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == '@' {
                let mut cnt: u8 = 0;
                for (dr, dc) in NEIGHBOR_DIRS.iter() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < n as isize && nc >= 0 && nc < m as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            cnt += 1;
                        }
                    }
                }

                if cnt < 4 {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;

    let grid: Vec<Vec<char>> = text
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let grid: Vec<Vec<char>> = grid.into_iter().filter(|row| !row.is_empty()).collect();

    let ans = solve(&grid);
    println!("{ans}");

    Ok(())
}
