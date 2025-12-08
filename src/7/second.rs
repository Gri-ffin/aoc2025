use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn solve(diagram_with_s: &[String]) -> u128 {
    let cleaned: Vec<&String> = diagram_with_s.iter().filter(|s| !s.is_empty()).collect();
    if cleaned.is_empty() {
        return 0;
    }

    let start_line = cleaned[0];
    let start_chars: Vec<char> = start_line.chars().collect();

    let mut rows: Vec<Vec<char>> = cleaned
        .iter()
        .skip(1)
        .map(|s| s.chars().collect())
        .collect();

    if rows.is_empty() {
        return 0;
    }

    let width = std::cmp::max(
        start_chars.len(),
        rows.iter().map(|r| r.len()).max().unwrap(),
    );
    for r in &mut rows {
        if r.len() < width {
            r.resize(width, '.');
        }
    }

    let start_col = start_chars.iter().position(|&c| c == 'S');
    let sc = match start_col {
        Some(i) => i,
        None => return 0,
    };

    let mut b_current: Vec<u128> = vec![0; width];
    b_current[sc] = 1;

    for r in 0..rows.len() {
        let mut b_next: Vec<u128> = vec![0; width];
        let row = &rows[r];
        for c in 0..width {
            let cnt = b_current[c];
            if cnt == 0 {
                continue;
            }
            match row[c] {
                '.' => {
                    b_next[c] = b_next[c].saturating_add(cnt);
                }
                '^' => {
                    if c > 0 {
                        b_next[c - 1] = b_next[c - 1].saturating_add(cnt);
                    }
                    if c + 1 < width {
                        b_next[c + 1] = b_next[c + 1].saturating_add(cnt);
                    }
                }
                _ => {}
            }
        }
        b_current = b_next;
        if b_current.iter().all(|&x| x == 0) {
            break;
        }
    }

    b_current.iter().sum()
}

fn main() -> io::Result<()> {
    let path = Path::new("src/input.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let ans = solve(&lines);
    println!("{}", ans);
    Ok(())
}
