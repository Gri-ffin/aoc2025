use std::error::Error;
use std::fs;
use std::str::FromStr;

fn has_repeating_pattern(num: u64) -> bool {
    let s = num.to_string();
    let n = s.len();

    for period_len in 1..=(n / 2) {
        if n % period_len == 0 {
            let pattern = &s[0..period_len];
            let repeat_count = n / period_len;
            let reconstructed_s = pattern.repeat(repeat_count);
            if reconstructed_s == s {
                return true;
            }
        }
    }

    false
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
                if has_repeating_pattern(x) {
                    ans += x;
                }
            }
        }
    }

    println!("{}", ans);
    Ok(())
}
