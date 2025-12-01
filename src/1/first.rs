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
            'L' => dial = modulo(dial - num),
            'R' => dial = modulo(dial + num),
            _ => {}
        }

        if dial == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
