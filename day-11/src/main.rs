use inpt::{inpt, Inpt};
use num::integer::lcm;
use std::collections::vec_deque::VecDeque;
use std::io::{self, BufRead};

type WorryLevel = i64;
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

    let worry_cap = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .reduce(lcm)
        .unwrap();

    // Figure out the monkey buisness 🙄
    let mut inspections_per_monkey: Vec<i64> = monkeys.iter().map(|_| 0).collect();
    // FORGIVE ME FOR THIS NESTING 😭😭😭
    const N_ROUNDS: usize = 10000;
    for round in 0..N_ROUNDS {
        // One round:
        for i in 0..monkeys.len() {
            // Monkey's turn:
            //eprintln!("Monkey {i}");
            // do this to prevent dance around with mutable borrows.
            let n_items = monkeys[i].items.len();
            for _ in 0..n_items {
                let (dest, item) = {
                    let monkey = &mut monkeys[i];
                    let mut item = monkey.items.pop_front().unwrap();
                    inspections_per_monkey[i] += 1;
                    //eprintln!("  Monkey inspects an item with a worry level of {item}.");
                    // inspect the item
                    item = monkey.inspect(item);
                    //eprintln!("    Worry level is {0:?} to {item}.", monkey.operation);

                    // After each monkey inspect an item...
                    // but before it tests the worry level...
                    //eprintln!( "    Monkey gets bored with item. Worry level is divided by 3 to {item}");

                    item %= worry_cap;

                    if (item % monkey.divisor) == 0 {
                        //eprintln!("    Current worry level is divisible by {}", monkey.divisor);
                        (monkey.throw_if_true, item)
                    } else {
                        //eprintln!( "    Current worry level is not divisible by {}", monkey.divisor);
                        (monkey.throw_if_false, item)
                    }
                };
                //eprintln!("    Item with worry level {item} is thrown to {dest}");
                monkeys[dest].items.push_back(item);
            }
        }

        let when = vec![
            1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
        ];
        if let Some(n) = when.iter().find(|&r| *r == round + 1) {
            eprintln!("== After Round {n} ==");
            for (i, times) in inspections_per_monkey.iter().enumerate() {
                eprintln!("Monkey {i} inspected items {times} times");
            }
        }
    }

    // DONE
    inspections_per_monkey.sort_by_key(|n| -n);
    let monkey_business = inspections_per_monkey[0] * inspections_per_monkey[1];
    println!("{monkey_business}");
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
