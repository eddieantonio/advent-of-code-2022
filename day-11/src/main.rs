use inpt::{inpt, Inpt};
use std::collections::vec_deque::VecDeque;
use std::io::{self, BufRead};

type WorryLevel = i32;
type MonkeyID = usize;

#[derive(Debug)]
struct Monkey {
    /// Worry level for each item a monkey is currently holding
    /// in order that it will be inspected.
    items: VecDeque<WorryLevel>,
    /// How worry level changes as the monkey inspects an item.
    operation: Expr,
    /// How monkey uses my worry level to to decide where to throw an item next
    divisor: WorryLevel,
    throw_if_true: MonkeyID,
    throw_if_false: MonkeyID,
}

#[derive(Inpt, Debug, PartialEq, Eq)]
enum Expr {
    #[inpt(regex = r"\s*[+]\s*")]
    Add(#[inpt(before)] Operand, #[inpt(after)] Operand),
    #[inpt(regex = r"\s*[*]\s*")]
    Multiply(#[inpt(before)] Operand, #[inpt(after)] Operand),
}

#[derive(Inpt, Debug, PartialEq, Eq, Copy, Clone)]
enum Operand {
    #[inpt(regex = r"old")]
    Old,
    #[inpt(regex = r"(\d+)")]
    Literal(WorryLevel),
}

fn main() {
    let mut monkeys = parse_monkeys();

    // Debug: Print all the monkeys!
    for (i, monkey) in monkeys.iter().enumerate() {
        eprintln!("Monkey {i}");
        eprintln!("  Starting items: {:?}", monkey.items);
        eprintln!("  Operation: new = {:?}", monkey.operation);
        eprintln!("  Test: divisible by {:?}", monkey.divisor);
        eprintln!("    If true: throw to monkey {:?}", monkey.throw_if_true);
        eprintln!("    If false: throw to monkey {:?}", monkey.throw_if_false);
        eprintln!();
    }

    // Figure out the monkey buisness 🙄
    eprintln!("===\n");

    // One round:
    for i in 0..monkeys.len() {
        eprintln!("Monkey {i}");
        let (dest, item) = {
            // Monkey's turn:
            let monkey = &mut monkeys[i];

            let mut item = monkey.items.pop_front().expect("Should have an item");
            eprintln!("  Monkey inspects an item with a worry level of {item}.");
            // inspect the item
            item = monkey.inspect(item);
            eprintln!("    Worry level is {0:?} to {item}.", monkey.operation);

            // After each monkey inspect an item...
            // but before it tests the worry level...
            let item = item / 3;
            eprintln!("    Monkey gets bored with item. Worry level is divided by 3 to {item}");

            if (item % monkey.divisor) == 0 {
                eprintln!("    Current worry level is divisible by {}", monkey.divisor);
                (monkey.throw_if_true, item)
            } else {
                eprintln!(
                    "    Current worry level is not divisible by {}",
                    monkey.divisor
                );
                (monkey.throw_if_false, item)
            }
        };
        eprintln!("    Item with worry level {item} is thrown to {dest}");
        monkeys[dest].items.push_back(item);
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(_) = lines.next() {
        // Expecting "Monkey" line.
        // Ignore it, we already know the monkey.

        // "Starting items: " line
        let line = lines.next().unwrap().unwrap();
        let (_, list) = line.split_once(':').unwrap();
        let items = parse_list(list.trim());

        // "Operation: new = ...." line
        let line = lines.next().unwrap().unwrap();
        let (_, expr) = line.split_once('=').unwrap();
        let operation: Expr = inpt(expr.trim()).unwrap();

        // "Test" line...
        let line = lines.next().unwrap().unwrap();
        let (_, num) = line.split_once("by").unwrap();
        let divisor: WorryLevel = num.trim().parse().unwrap();

        // "If true" line...
        let line = lines.next().unwrap().unwrap();
        let (_, num) = line.split_once("monkey").unwrap();
        let throw_if_true: MonkeyID = num.trim().parse().unwrap();

        // "If false" line...
        let line = lines.next().unwrap().unwrap();
        let (_, num) = line.split_once("monkey").unwrap();
        let throw_if_false: MonkeyID = num.trim().parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            throw_if_true,
            throw_if_false,
        });

        // empty line that separates monkeys
        if lines.next().is_none() {
            break;
        }
    }

    monkeys
}

fn parse_list(s: &str) -> VecDeque<WorryLevel> {
    s.split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

impl Monkey {
    fn inspect(&self, item: WorryLevel) -> WorryLevel {
        self.operation.evaluate(item)
    }
}

impl Expr {
    fn evaluate(&self, old: WorryLevel) -> WorryLevel {
        use Expr::*;
        match self {
            Add(a, b) => a.evaluate(old) + b.evaluate(old),
            Multiply(a, b) => a.evaluate(old) * b.evaluate(old),
        }
    }
}

impl Operand {
    fn evaluate(self, old: WorryLevel) -> WorryLevel {
        use Operand::*;
        match self {
            Old => old,
            Literal(x) => x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        use Expr::*;
        use Operand::*;

        let op = inpt::<Expr>("old * 19").unwrap();
        assert_eq!(Multiply(Old, Literal(19)), op);
        assert_eq!(2 * 19, op.evaluate(2));

        let op = inpt::<Expr>("old + 3").unwrap();
        assert_eq!(Add(Old, Literal(3)), op);
        assert_eq!(2 + 3, op.evaluate(2));

        let op = inpt::<Expr>("old * old").unwrap();
        assert_eq!(Multiply(Old, Old), op);
        assert_eq!(3 * 3, op.evaluate(3));
    }
}
