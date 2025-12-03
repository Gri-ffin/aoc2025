use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;
    let mut ans: u64 = 0;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let num: String = line.parse()?;
        let mut l_max_num: u8 = 0;
        let mut r_max_num: u8 = 0;

        let digs: Vec<u8> = num
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as u8)
            .collect();

        for i in 0..(digs.len() - 1) {
            let cur1 = digs[i];
            if cur1 > l_max_num {
                r_max_num = digs[i + 1];
                for j in (i + 1)..digs.len() {
                    let cur2 = digs[j];
                    if cur2 >= r_max_num {
                        l_max_num = cur1;
                        r_max_num = cur2;
                    }
                }
            }
        }

        let s_dig = l_max_num.to_string() + &r_max_num.to_string();
        ans += s_dig.parse::<u64>()?;
    }

    println!("{}", ans);
    Ok(())
}
