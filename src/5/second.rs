use std::error::Error;
use std::fs;

fn solve(fresh_oranges: &Vec<(u64, u64)>) -> u64 {
    let mut ans = 0;
    for &(start, end) in fresh_oranges {
        ans += end - start + 1;
    }
    ans
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for next_range in ranges.iter().skip(1) {
        let next_start = next_range.0;
        let next_end = next_range.1;

        if next_start <= current_end + 1 {
            current_end = current_end.max(next_end);
        } else {
            merged.push((current_start, current_end));

            current_start = next_start;
            current_end = next_end;
        }
    }

    merged.push((current_start, current_end));

    merged
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;

    let parts: Vec<&str> = text.split("\n\n").collect();

    let fresh_ranges: Result<Vec<(u64, u64)>, Box<dyn Error>> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let range_parts: Vec<&str> = line.trim().split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!("Invalid range format: {}", line).into());
            }
            let start = range_parts[0].parse::<u64>()?;
            let end = range_parts[1].parse::<u64>()?;

            Ok((start, end))
        })
        .collect();

    let fresh_ranges = fresh_ranges?;
    let merged_ranges = merge_ranges(fresh_ranges);
    let ans = solve(&merged_ranges);
    println!("{ans}");

    Ok(())
}
