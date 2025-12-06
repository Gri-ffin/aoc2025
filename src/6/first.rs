use std::error::Error;
use std::fs;

fn solve(matrix: &Vec<Vec<u64>>, ops: &Vec<char>) -> u64 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut ans = 0;
    for c in 0..m {
        let op = ops[c];
        let mut cur = 0;
        if op == '*' {
            cur = 1;
        }
        for r in 0..n {
            let num = matrix[r][c];
            if op == '*' {
                cur *= num;
            } else {
                cur += num;
            }
        }
        ans += cur;
    }
    ans
}

type Matrix = Vec<Vec<u64>>;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;

    let mut all_lines: Vec<String> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let last_line_str = all_lines
        .pop()
        .ok_or_else(|| "Input file is empty or contains only newlines.".to_string())?;

    let matrix_data_lines = all_lines;

    let ops: Vec<char> = last_line_str
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    let matrix: Result<Matrix, Box<dyn Error>> = matrix_data_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().map_err(|e| e.into()))
                .collect::<Result<Vec<u64>, Box<dyn Error>>>()
        })
        .collect::<Result<Matrix, Box<dyn Error>>>();

    let matrix = matrix?;

    let ans = solve(&matrix, &ops);

    println!("{}", ans);

    Ok(())
}
