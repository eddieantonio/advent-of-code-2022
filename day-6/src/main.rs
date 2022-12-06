use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    let stream = buffer.trim();
    let answer = start_packet(stream).unwrap();

    println!("{answer}");
}

pub fn start_packet(s: &str) -> Option<usize> {
    start_distinct::<4>(s)
}

fn start_distinct<const PREFIX_SIZE: usize>(s: &str) -> Option<usize> {
    let mut buffer = ['\0'; PREFIX_SIZE];

    for (i, c) in s.chars().enumerate() {
        buffer[i % PREFIX_SIZE] = c;
        if i < PREFIX_SIZE {
            continue;
        }

        let set: HashSet<_> = buffer.iter().collect();
        if set.len() == PREFIX_SIZE {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(Some(7), start_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), start_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), start_packet("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), start_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), start_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
