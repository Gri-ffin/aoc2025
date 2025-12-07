use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/input.txt")?;
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        println!("0");
        return Ok(());
    }

    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);
    for row in &mut grid {
        if row.len() < width {
            row.extend(std::iter::repeat('.').take(width - row.len()));
        }
    }

    let mut start_r = None;
    let mut start_c = None;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_r = Some(r);
                start_c = Some(c);
                break;
            }
        }
        if start_r.is_some() {
            break;
        }
    }

    let (sr, sc) = match (start_r, start_c) {
        (Some(r), Some(c)) => (r, c),
        _ => {
            return Ok(());
        }
    };

    let rows = grid.len();
    let cols = width;

    let mut active: HashSet<usize> = HashSet::new();
    active.insert(sc);

    let mut splits: usize = 0;

    for r in (sr + 1)..rows {
        let mut next_active: HashSet<usize> = HashSet::new();
        for &c in &active {
            if c >= cols {
                continue;
            }
            match grid[r][c] {
                '^' => {
                    splits += 1;
                    if c > 0 {
                        next_active.insert(c - 1);
                    }
                    if c + 1 < cols {
                        next_active.insert(c + 1);
                    }
                }
                _ => {
                    next_active.insert(c);
                }
            }
        }
        if next_active.is_empty() {
            break;
        }
        active = next_active;
    }

    println!("{}", splits);
    Ok(())
}
