use std::collections::HashSet;
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

fn solve(grid: &mut Vec<Vec<char>>) -> u32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut ans = 0;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
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
                    set.insert((r, c));
                    ans += 1;
                }
            }
        }
    }
    for (r, c) in set {
        grid[r][c] = '.';
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;

    let grid: Vec<Vec<char>> = text
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut grid: Vec<Vec<char>> = grid.into_iter().filter(|row| !row.is_empty()).collect();

    let mut ans = 0;
    loop {
        let cur = solve(&mut grid);
        if cur > 0 {
            ans += cur;
        } else {
            break;
        }
    }
    println!("{ans}");

    Ok(())
}
