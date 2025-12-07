use std::error::Error;
use std::fs;

fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut problems = Vec::new();
    let mut col_idx = 0;

    while col_idx < max_width {
        let is_separator = lines
            .iter()
            .all(|line| col_idx >= line.len() || line.chars().nth(col_idx).unwrap() == ' ');

        if is_separator {
            col_idx += 1;
            continue;
        }

        let start = col_idx;
        while col_idx < max_width {
            let is_sep = lines
                .iter()
                .all(|line| col_idx >= line.len() || line.chars().nth(col_idx).unwrap() == ' ');

            if is_sep {
                break;
            }
            col_idx += 1;
        }

        problems.push((start, col_idx));
    }

    let mut grand_total = 0u64;
    let operator_row = lines.len() - 1;

    for &(start, end) in &problems {
        let mut numbers = Vec::new();

        for col in (start..end).rev() {
            let mut digit_chars = Vec::new();

            for row_idx in 0..operator_row {
                if col < lines[row_idx].len() {
                    let ch = lines[row_idx].chars().nth(col).unwrap();
                    println!("({ch})");
                    if ch.is_ascii_digit() {
                        digit_chars.push(ch);
                    }
                }
            }

            if !digit_chars.is_empty() {
                let num_str: String = digit_chars.iter().collect();
                if let Ok(num) = num_str.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        let mut operator = ' ';
        for col in start..end {
            if col < lines[operator_row].len() {
                let ch = lines[operator_row].chars().nth(col).unwrap();
                if ch == '+' || ch == '*' {
                    operator = ch;
                    break;
                }
            }
        }

        if !numbers.is_empty() && (operator == '+' || operator == '*') {
            let result: u64 = if operator == '+' {
                numbers.iter().sum()
            } else {
                numbers.iter().product()
            };

            grand_total += result;
        }
    }

    grand_total
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;
    let ans = solve(&text);
    println!("{}", ans);
    Ok(())
}

