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

    let all_rucksacks: Vec<Rucksack> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().into())
        .collect();

    let answer = all_rucksacks[..]
        .chunks(3)
        .map(|group| {
            let all = grand_intersection(group.iter().map(|x| x.items()));
            assert_eq!(1, all.len());

            all.into_iter().next().unwrap().value()
        })
        .sum::<i32>();

    println!("{answer}");
    Ok(())
}

fn grand_intersection<'a, T: Iterator<Item = Items<'a>>>(collections: T) -> HashSet<Item> {
    collections
        .map(|items| items.collect::<HashSet<Item>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .unwrap()
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
    fn items(&self) -> Items<'_> {
        self.both_contents.chars().into()
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

impl<'a> From<Chars<'a>> for Items<'a> {
    fn from(iterator: Chars<'a>) -> Items<'a> {
        Items(iterator)
    }
}
