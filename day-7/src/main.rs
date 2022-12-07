use inpt::{inpt, Inpt};
use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Inpt, Debug)]
enum Line {
    #[inpt(regex = r"[$]\s+cd\s+(.+)")]
    Cd(String),
    #[inpt(regex = r"[$]\s+ls")]
    Ls,
    #[inpt(regex = r"dir\s+(.+)")]
    Directory(String),
    #[inpt(regex = r"(\d+)\s+.+")]
    File(usize),
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| inpt::<Line>(line.trim()).unwrap())
        .collect();

    println!("{lines:?}");
}

struct Calculation(VecDeque<Line>, usize);

fn calculate_size(mut lines: VecDeque<Line>) -> Calculation {
    let Some(Line::Ls) = lines.pop_front() else {
        panic!("Did not see ls");
    };

    let mut current_size = 0;
    loop {
        let line = lines.pop_front();
        match line {
            Some(Line::Cd(dir)) if &dir == ".." => {
                return Calculation(lines, current_size);
            }
            Some(Line::File(size)) => current_size += size,
            Some(Line::Directory(_)) => {
                // Don't need to do anything for a directory
            }
            Some(Line::Cd(name)) => {
                let Calculation(new_lines, size) = calculate_size(lines);
                lines = new_lines;
                current_size += size;
            }
            _ => unimplemented!("{line:?}, {lines:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        use Line::*;
        match inpt::<Line>("$ cd /") {
            Ok(Cd(dir)) => assert_eq!("/", &dir),
            err => panic!("{err:?}"),
        }

        match inpt::<Line>("$ ls") {
            Ok(Ls) => (),
            err => panic!("{err:?}"),
        }

        match inpt::<Line>("dir e") {
            Ok(Directory(name)) => assert_eq!("e", &name),
            err => panic!("{err:?}"),
        }

        match inpt::<Line>("62596 h.lst") {
            Ok(File(size)) => assert_eq!(62596, size),
            err => panic!("{err:?}"),
        }
    }

    #[test]
    fn test_leaf_directory() {
        let lines: VecDeque<_> = vec!["$ ls", "584 i", "$ cd .."]
            .into_iter()
            .map(|line| inpt::<Line>(line).unwrap())
            .collect();

        let Calculation(lines, size) = calculate_size(lines);
        assert_eq!(584, size);
        assert_eq!(0, lines.len());
    }

    #[test]
    fn test_nested_directories() {
        let lines = vec![
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
        ];
        let lines: VecDeque<_> = lines
            .into_iter()
            .map(|line| inpt::<Line>(line).unwrap())
            .collect();

        let Calculation(lines, size) = calculate_size(lines);
        assert_eq!(94853, size);
        assert_eq!(0, lines.len());
    }

    #[test]
    fn test_herp() {
        let lines = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let lines: Vec<_> = lines
            .into_iter()
            .map(|line| inpt::<Line>(line).unwrap())
            .collect();
    }
}
