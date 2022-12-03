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

    let mut all_elves: Vec<_> = elves
        .into_iter()
        .map(|pack| pack.into_iter().sum::<i32>())
        .collect();

    all_elves.sort();

    let top_three = all_elves.into_iter().rev().take(3).sum::<i32>();
    println!("{top_three}");

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
