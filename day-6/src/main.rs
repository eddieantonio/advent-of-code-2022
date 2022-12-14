use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    let stream = buffer.trim();
    let answer = start_message(stream).unwrap();

    println!("{answer}");
}

pub fn start_packet(s: &str) -> Option<usize> {
    start_distinct::<4>(s)
}

pub fn start_message(s: &str) -> Option<usize> {
    start_distinct::<14>(s)
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
    fn test_start_packet() {
        assert_eq!(Some(7), start_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), start_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), start_packet("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), start_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), start_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_start_message() {
        assert_eq!(Some(19), start_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(23), start_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), start_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(29), start_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(26), start_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
