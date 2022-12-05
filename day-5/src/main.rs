use inpt::{inpt, Inpt};
use std::io::{self, BufRead};

#[derive(Inpt, Debug)]
#[inpt(regex = r"move (\d+) from (\d+) to (\d+)")]
struct Move {
    quantity: i32,
    source: usize,
    destination: usize,
}

#[derive(Inpt, Debug, Copy, Clone)]
#[inpt(regex = r"\[(\w)\]")]
struct Crate(char);

trait VecExt<T> {
    fn last(&self) -> T;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let mut crate_lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim() == "" {
            break;
        }

        crate_lines.push(line.to_owned());
    }

    let last_line = crate_lines.pop().expect("There must be at least one line");
    let indices: Vec<usize> = inpt(&last_line).unwrap();
    let size = indices[indices.len() - 1];

    // awful parsing algorithm
    let mut stacks = Vec::new();
    // Stacks are one-indexed, so add a padding struct at the beginning.
    stacks.push(vec![]);
    for stack_id in 0..size {
        let mut stack = Vec::new();
        let start_index = stack_id * 4;
        let end_index = start_index + 3;
        let chunk = start_index..end_index;

        for line in crate_lines.iter().rev() {
            let Some(crate_str) = line.get(chunk.clone()) else {
                break;
            };
            assert_eq!(3, crate_str.len());
            if crate_str.trim() == "" {
                continue;
            }

            let Crate(c) = inpt(crate_str).unwrap();
            stack.push(c);
        }

        stacks.push(stack);
    }

    // Parse moves.
    let moves: Vec<Move> = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            inpt(&line).unwrap()
        })
        .collect();

    for m in moves.into_iter() {
        for _ in 0..m.quantity {
            let c = stacks[m.source].pop().unwrap();
            stacks[m.destination].push(c);
        }
    }

    let message: String = stacks
        .into_iter()
        .skip(1)
        .map(|stack| stack.last())
        .collect();

    println!("{message}");

    Ok(())
}

impl<T> VecExt<T> for Vec<T>
where
    T: Copy,
{
    fn last(&self) -> T {
        self[self.len() - 1]
    }
}
