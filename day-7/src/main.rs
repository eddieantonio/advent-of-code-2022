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
    let mut lines: VecDeque<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| inpt::<Line>(line.trim()).unwrap())
        .collect();

    lines.pop_front();
    let Calculation(_, root_size, mut dirs) = calculate_size(lines, vec![]);
    let space_available = 70000000;
    let space_required = 30000000;

    let unused_space = space_available - root_size;
    assert!(unused_space < space_required);

    dirs.sort_by_key(|DirectorySize(_, size)| *size);

    let DirectorySize(name, size) = dirs
        .into_iter()
        .find(|DirectorySize(_, size)| size + unused_space > space_required)
        .unwrap();

    println!("{name}: {size}");
}

struct DirectorySize(String, usize);
struct Calculation(VecDeque<Line>, usize, Vec<DirectorySize>);

fn calculate_size(mut lines: VecDeque<Line>, mut dirs: Vec<DirectorySize>) -> Calculation {
    let Some(Line::Ls) = lines.pop_front() else {
        panic!("Did not see ls");
    };

    let mut current_size = 0;
    loop {
        let line = lines.pop_front();
        match line {
            Some(Line::Cd(dir)) if &dir == ".." => {
                return Calculation(lines, current_size, dirs);
            }
            Some(Line::File(size)) => current_size += size,
            Some(Line::Directory(_)) => {
                // Don't need to do anything for a directory
            }
            Some(Line::Cd(name)) => {
                let Calculation(new_lines, size, new_dirs) = calculate_size(lines, dirs);
                lines = new_lines;
                dirs = new_dirs;
                current_size += size;
                dirs.push(DirectorySize(name, size));
            }
            None => {
                return Calculation(lines, current_size, dirs);
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

        let Calculation(lines, size, dirs) = calculate_size(lines, vec![]);
        assert_eq!(584, size);
        assert_eq!(0, lines.len());
        assert_eq!(0, dirs.len());
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

        let Calculation(lines, size, mut dirs) = calculate_size(lines, vec![]);
        assert_eq!(94853, size);
        assert_eq!(0, lines.len());
        assert_eq!(1, dirs.len());
        let DirectorySize(name, e_size) = dirs.pop().unwrap();
        assert_eq!("e", &name);
        assert_eq!(584, e_size);
    }

    #[test]
    fn test_given_input() {
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
        let mut lines: VecDeque<_> = lines
            .into_iter()
            .map(|line| inpt::<Line>(line).unwrap())
            .collect();

        lines.pop_front();
        let Calculation(lines, size, mut dirs) = calculate_size(lines, vec![]);
        assert_eq!(0, lines.len());
        assert_eq!(48381165, size);
        assert_eq!(3, dirs.len());

        // part 1
        let total_size = dirs
            .iter()
            .map(|DirectorySize(_, size)| size)
            .filter(|&&size| size < 100000)
            .sum::<usize>();
        assert_eq!(95437, total_size);

        // part 2
        let space_available = 70000000;
        let space_required = 30000000;
        let unused_space = space_available - size;
        assert!(unused_space < space_required);

        dirs.sort_by_key(|DirectorySize(_, size)| *size);

        let DirectorySize(name, size) = dirs
            .into_iter()
            .find(|DirectorySize(_, size)| size + unused_space > space_required)
            .unwrap();
        assert_eq!(24933642, size);
        assert_eq!("d", &name);
    }
}
