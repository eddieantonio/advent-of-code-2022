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
            elves.last().push(value);
        }
    }

    let Some((index, calories)) = elves
        .into_iter()
        .map(|pack| pack.into_iter().sum::<i32>())
        .enumerate()
        .max_by_key(|(_, elf)| *elf) else {
            panic!("should not happen");
        };

    let the_elf = index + 1;
    println!("Elf {the_elf} is carrying {calories} calories.");

    Ok(())
}

trait Last<T> {
    fn last(&mut self) -> &mut T;
}

impl<T> Last<T> for Vec<T> {
    fn last(&mut self) -> &mut T {
        let idx = self.len() - 1;
        &mut self[idx]
    }
}
