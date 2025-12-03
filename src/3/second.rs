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

        const K: usize = 12;
        let n = line.len();
        let mut del_left = n - K;
        let mut stack: Vec<char> = Vec::with_capacity(K);
        for dig in line.chars() {
            while !stack.is_empty() && del_left > 0 {
                if *stack.last().unwrap() < dig {
                    stack.pop();
                    del_left -= 1;
                } else {
                    break;
                }
            }
            stack.push(dig);
        }
        while stack.len() > K {
            stack.pop();
        }
        let num: String = stack.into_iter().collect();
        ans += num.parse::<u64>()?;
    }

    println!("{}", ans);
    Ok(())
}
