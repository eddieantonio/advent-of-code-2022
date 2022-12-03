use std::collections::HashSet;
use std::io::{self, BufRead};

struct Rucksack {
    both_contents: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let answer = stdin
        .lock()
        .lines()
        .map(|line| {
            let rucksack: Rucksack = line.unwrap().trim().into();
            let left_half: HashSet<_> = rucksack.left_half().chars().collect();

            let mut needle = None;
            for item in rucksack.right_half().chars() {
                if left_half.contains(&item) {
                    needle = Some(item);
                    break;
                }
            }

            value(needle.unwrap())
        })
        .sum::<i32>();

    println!("{answer}");
    Ok(())
}

fn value(c: char) -> i32 {
    let ord: u32 = c.into();

    ////////////////// 0b00100000;
    let letter = ord & 0b00011111;
    let displacement = if (ord & 0b00100000) != 0 { 0 } else { 26 };
    (letter + displacement) as i32
}

impl Rucksack {
    fn left_half(&self) -> &str {
        &self.both_contents[0..self.half_len()]
    }

    fn right_half(&self) -> &str {
        &self.both_contents[self.half_len()..self.len()]
    }

    fn len(&self) -> usize {
        self.both_contents.len()
    }

    fn half_len(&self) -> usize {
        assert_eq!(0, self.len() % 2);
        self.len() / 2
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Rucksack {
        Rucksack {
            both_contents: s.to_owned(),
        }
    }
}
