use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut elves = vec![vec![]];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim() == "" {
            elves.push(vec![]);
        } else {
            let value: i32 = line.parse()?;
            let last_idx = elves.len() - 1;
            elves[last_idx].push(value);
        }
    }

    let calories_per_elf = elves.into_iter().map(|pack| pack.into_iter().sum::<i32>());
    let mut index_of_maximum = 0;
    let mut value_of_maximum = None;
    for (i, amount) in calories_per_elf.enumerate() {
        value_of_maximum = match value_of_maximum {
            None => Some(amount),
            Some(previous) => {
                if amount > previous {
                    index_of_maximum = i;
                    Some(amount)
                } else {
                    Some(previous)
                }
            }
        }
    }

    let the_elf = index_of_maximum + 1;
    println!("{the_elf}");

    Ok(())
}
