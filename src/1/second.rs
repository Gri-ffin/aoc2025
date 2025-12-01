use std::error::Error;
use std::fs;

fn modulo(a: i32) -> i32 {
    ((a % 100) + 100) % 100
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("src/input.txt")?;
    let mut dial = 50;
    let mut ans = 0;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let num: i32 = line[1..].parse()?;

        match dir {
            'L' => {
                let mut k = dial % 100;
                if k == 0 {
                    k = 100;
                }
                if k <= num {
                    ans += 1 + (num - k) / 100;
                }

                dial = modulo(dial - num);
            }

            'R' => {
                let mut k = (100 - dial) % 100;
                if k == 0 {
                    k = 100;
                }
                if k <= num {
                    ans += 1 + (num - k) / 100;
                }

                dial = modulo(dial + num);
            }

            _ => unreachable!(),
        }
    }

    println!("{}", ans);
    Ok(())
}
