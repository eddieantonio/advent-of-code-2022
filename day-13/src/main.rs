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

    let mut packets: Vec<_> = Vec::new();
    loop {
        let line_1 = lines.next().unwrap().unwrap();
        let line_2 = lines.next().unwrap().unwrap();

        let left = parse(&line_1);
        let right = parse(&line_2);

        packets.push(left);
        packets.push(right);

        if lines.next().is_none() {
            break;
        }
    }

    packets.push(parse("[[2]]"));
    packets.push(parse("[[6]]"));

    packets.sort_by(|left, right| {
        if compare(left, right) == Some(true) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut index_start = None;
    let mut index_end = None;
    for (i, p) in packets.iter().enumerate() {
        let yes_this_is_wasteful = p.to_string();
        if &yes_this_is_wasteful == "[[2]]" {
            index_start = Some(i + 1);
        }

        if &yes_this_is_wasteful == "[[6]]" {
            index_end = Some(i + 1);
        }
    }

    let index_start = index_start.unwrap();
    let index_end = index_end.unwrap();
    println!("{}", index_start * index_end);
}

fn compare(left: &Data, right: &Data) -> Option<bool> {
    match (left, right) {
        (Data::Int(a), Data::Int(b)) => match a.cmp(b) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (Data::List(a), Data::List(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();

            loop {
                match (a.next(), b.next()) {
                    (Some(v_a), Some(v_b)) => match compare(v_a, v_b) {
                        result @ Some(_) => return result,
                        _ => continue,
                    },
                    (None, Some(_)) => {
                        return Some(true);
                    }
                    (Some(_), None) => {
                        return Some(false);
                    }
                    (None, None) => {
                        return None;
                    }
                }
            }
        }
        (Data::Int(a), b) => {
            let l = Data::List(vec![Data::Int(*a)]);
            compare(&l, b)
        }
        (a, Data::Int(b)) => {
            let l = Data::List(vec![Data::Int(*b)]);
            compare(a, &l)
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
            self.accept(']');
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
