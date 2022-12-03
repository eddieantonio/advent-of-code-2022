use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::Chars;

struct Rucksack {
    both_contents: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Item(char);

struct Items<'a>(Chars<'a>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let answer = stdin
        .lock()
        .lines()
        .map(|line| {
            let rucksack: Rucksack = line.unwrap().trim().into();
            let left_half: HashSet<_> = rucksack.left_half().collect();

            let needle = rucksack
                .right_half()
                .find_map(|item| left_half.contains(&item).then_some(item));

            needle.unwrap().value()
        })
        .sum::<i32>();

    println!("{answer}");
    Ok(())
}

impl Item {
    fn value(self) -> i32 {
        let ord: u32 = self.0.into();

        let letter = ord & 0b00011111;
        let displacement = if (ord & 0b00100000) != 0 { 0 } else { 26 };
        (letter + displacement) as i32
    }
}

impl Rucksack {
    fn left_half(&self) -> Items {
        Items(self.both_contents[0..self.half_len()].chars())
    }

    fn right_half(&self) -> Items {
        Items(self.both_contents[self.half_len()..self.len()].chars())
    }

    fn len(&self) -> usize {
        self.both_contents.len()
    }

    fn half_len(&self) -> usize {
        assert_eq!(0, self.len() % 2);
        self.len() / 2
    }
}

impl<'a> Iterator for Items<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        self.0.next().map(Item)
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Rucksack {
        Rucksack {
            both_contents: s.to_owned(),
        }
    }
}
