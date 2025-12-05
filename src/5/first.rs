use std::error::Error;
use std::fs;

fn solve(fresh_oranges: &Vec<(u64, u64)>, available_ids: &Vec<u64>) -> u64 {
    let mut ans = 0;
    for &id in available_ids {
        let is_fresh = fresh_oranges
            .iter()
            .any(|(start, end)| id >= *start && id <= *end);

        if is_fresh {
            ans += 1;
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;

    let parts: Vec<&str> = text.split("\n\n").collect();

    if parts.len() != 2 {
        return Err("Input format error: Expected two sections separated by a blank line.".into());
    }

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

    let fresh_oranges = fresh_ranges?;

    let available_ids: Result<Vec<u64>, Box<dyn Error>> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            // Parse each line as a single ingredient ID
            line.trim().parse::<u64>().map_err(|e| e.into())
        })
        .collect();

    let available_ids = available_ids?;
    let ans = solve(&fresh_oranges, &available_ids);
    println!("{ans}");

    Ok(())
}
