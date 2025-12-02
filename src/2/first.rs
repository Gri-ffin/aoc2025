use std::error::Error;
use std::fs;
use std::str::FromStr;

fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let length = s.len();

    if length == 0 || length % 2 != 0 {
        return false;
    }

    let half_length = length / 2;
    let first_half = &s[0..half_length];
    let second_half = &s[half_length..length];

    first_half == second_half
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;
    let mut ans: u64 = 0;

    if let Some(line) = text.lines().next() {
        for range_str in line.split(',') {
            let parts: Vec<&str> = range_str.split('-').collect();
            let start = u64::from_str(parts[0].trim())?;
            let end = u64::from_str(parts[1].trim())?;
            for x in start..=end {
                if is_repeated_twice(x) {
                    ans += x;
                }
            }
        }
    }

    println!("{}", ans);
    Ok(())
}
