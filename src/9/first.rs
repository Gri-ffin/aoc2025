use std::fs;
use std::io;

fn solve(coords: &[(u64, u64)]) -> u64 {
    let n = coords.len();
    let mut ans = 0;

    for i in 0..n {
        let (x_i, y_i) = coords[i];

        for j in (i + 1)..n {
            let (x_j, y_j) = coords[j];

            if x_j == x_i && y_j == y_i {
                continue;
            }

            let width = x_i.abs_diff(x_j) + 1;
            let height = y_i.abs_diff(y_j) + 1;

            let current_area = width * height;
            ans = ans.max(current_area);
        }
    }
    ans
}
fn main() -> Result<(), io::Error> {
    let file_path = "src/input.txt";

    let content = fs::read_to_string(file_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read file {}: {}", file_path, e),
        )
    })?;

    let mut coordinates: Vec<(u64, u64)> = Vec::new();

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed_line.split(',').collect();

        if parts.len() != 2 {
            eprintln!("Skipping line with invalid format: {}", trimmed_line);
            continue;
        }

        let x = match parts[0].trim().parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "Skipping line due to X-coordinate parse error: {}",
                    trimmed_line
                );
                continue;
            }
        };

        let y = match parts[1].trim().parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "Skipping line due to Y-coordinate parse error: {}",
                    trimmed_line
                );
                continue;
            }
        };

        coordinates.push((x, y));
    }

    let ans = solve(&coordinates);
    println!("{}", ans);

    Ok(())
}
