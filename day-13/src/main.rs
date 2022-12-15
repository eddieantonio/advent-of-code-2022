use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq)]
enum Data {
    List(Vec<Data>),
    Int(i64),
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    //let f = File::open("test-0.txt").unwrap();
    //let mut lines = BufReader::new(f).lines();

    let mut pairs: Vec<_> = Vec::new();
    loop {
        let line_1 = lines.next().unwrap().unwrap();
        let line_2 = lines.next().unwrap().unwrap();

        let left = parse(&line_1);
        let right = parse(&line_2);

        pairs.push((left, right));

        if lines.next().is_none() {
            break;
        }
    }

    let mut sum = 0;

    for (i, (left, right)) in pairs.iter().enumerate() {
        let index = i + 1;
        println!("== Pair {index} ==");
        if compare(left, right) == Some(true) {
            sum += index;
        }
        println!();
    }

    println!("{sum}");
}

fn compare(left: &Data, right: &Data) -> Option<bool> {
    _compare(left, right, 0)
}

fn indent(depth: usize) {
    for _ in 0..depth {
        print!(" ");
    }
}

fn _compare(left: &Data, right: &Data, depth: usize) -> Option<bool> {
    indent(depth);
    println!("- Compare {left} vs {right}");

    match (left, right) {
        (Data::Int(a), Data::Int(b)) => match a.cmp(b) {
            Ordering::Less => {
                indent(depth);
                println!(" - Left side is smaller, so inputs are in the right order");
                Some(true)
            }
            Ordering::Greater => {
                indent(depth);
                println!(" - Right side is smaller, so inputs are not in the right order");
                Some(false)
            }
            Ordering::Equal => None,
        },
        (Data::List(a), Data::List(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();

            loop {
                match (a.next(), b.next()) {
                    (Some(v_a), Some(v_b)) => match _compare(v_a, v_b, depth + 1) {
                        result @ Some(_) => return result,
                        _ => continue,
                    },
                    (None, Some(_)) => {
                        indent(depth);
                        println!(" - Left side ran out of items, so inputs are in the right order");
                        return Some(true);
                    }
                    (Some(_), None) => {
                        indent(depth);
                        println!(
                            " - Right side ran out of items, so inputs are not in the right order"
                        );
                        return Some(false);
                    }
                    (None, None) => {
                        return None;
                    }
                }
            }
        }
        (Data::Int(a), b) => {
            indent(depth);
            println!(" - Mixed types; convert left to [{a}] and retry comparison");
            let l = Data::List(vec![Data::Int(*a)]);
            _compare(&l, b, depth + 1)
        }
        (a, Data::Int(b)) => {
            indent(depth);
            println!(" - Mixed types; convert right to [{b}] and retry comparison");
            let l = Data::List(vec![Data::Int(*b)]);
            _compare(a, &l, depth + 1)
        }
    }
}

fn parse(s: &str) -> Data {
    let mut parser = Parser {
        input: s.as_bytes(),
    };

    parser.list()
}

struct Parser<'a> {
    input: &'a [u8],
}

impl<'a> Parser<'a> {
    fn list(&mut self) -> Data {
        self.accept('[');

        if self.peek() == Some(']') {
            return Data::List(vec![]);
        }

        let mut elements = vec![self.element()];

        while self.peek() == Some(',') {
            self.accept(',');
            elements.push(self.element());
        }
        self.accept(']');

        Data::List(elements)
    }

    fn element(&mut self) -> Data {
        match self.peek() {
            Some(c) if c.is_ascii_digit() => self.integer(),
            Some('[') => self.list(),
            _ => panic!("syntax error: {:?}", self.as_string()),
        }
    }

    fn integer(&mut self) -> Data {
        let mut v: Vec<char> = Vec::new();

        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            v.push(c);
            self.advance();
        }

        let s: String = v.into_iter().collect();

        Data::Int(s.parse().unwrap())
    }

    fn accept(&mut self, c: char) {
        if self.peek() != Some(c) {
            panic!("syntax error: {:?}", self.as_string());
        }
        self.advance();
    }

    fn peek(&self) -> Option<char> {
        self.input
            .iter()
            .next()
            .map(|&byte| char::from_u32(byte as u32).unwrap())
    }

    fn advance(&mut self) {
        self.input = &self.input[1..];
    }

    fn as_string(&self) -> &str {
        std::str::from_utf8(self.input).unwrap()
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Int(i) => write!(f, "{i}"),
            Data::List(l) => {
                write!(f, "[")?;

                match l.len() {
                    0 => (),
                    1 => {
                        write!(f, "{}", l[0])?;
                    }
                    _ => {
                        write!(f, "{}", l[0])?;
                        for item in l.iter().skip(1) {
                            write!(f, ",{}", item)?;
                        }
                    }
                }

                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "[1,1,3,1,1]";
        assert_eq!(
            Data::List(vec![
                Data::Int(1),
                Data::Int(1),
                Data::Int(3),
                Data::Int(1),
                Data::Int(1)
            ]),
            parse(input)
        );

        let input = "[[1],[2,3,4]]";
        assert_eq!(
            Data::List(vec![
                Data::List(vec![Data::Int(1)]),
                Data::List(vec![Data::Int(2), Data::Int(3), Data::Int(4),]),
            ]),
            parse(input)
        );

        let input = "[[[]]]";
        assert_eq!(
            Data::List(vec![Data::List(vec![Data::List(vec![])]),]),
            parse(input)
        );
    }
}
