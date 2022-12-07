use inpt::{inpt, Inpt};
use std::io::{self, BufRead};

#[derive(Inpt, Debug)]
#[inpt(regex = r"[$]\s+")]
struct CommandLine(#[inpt(after)] Command);

#[derive(Inpt, Debug)]
enum Command {
    #[inpt(regex = r"cd\s+(.+)")]
    Cd(String),
    #[inpt(regex = r"ls")]
    Ls,
}

#[derive(Debug, Inpt)]
enum Entry {
    #[inpt(regex = r"dir\s+(.+)")]
    Directory(String),
    #[inpt(regex = r"(\d+)\s+.+")]
    File(usize),
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    count_from_root(lines.into_iter());
}

fn count_from_root(mut lines: impl Iterator<Item = String>) {
    let first = lines.next().unwrap();
    let Ok(CommandLine(Command::Cd(dir))) = inpt::<CommandLine>(&first) else {
        panic!("First line bad: {first}")
    };

    if &dir != "/" {
        panic!("Not root directory");
    }

    count_directory(lines);
}

/// Assumes:
///  - previous line was $ cd <dir>
///  - current line is ls
///
fn count_directory(mut lines: impl Iterator<Item = String>) -> usize {
    let line = lines.next().unwrap();
    let Ok(CommandLine(Command::Ls)) = inpt::<CommandLine>(&line) else {
        panic!("Expected line to be ls but was: {line}")
    };

    let mut size = 0;

    // parse directory entries
    loop {
        let Some(line) = lines.next() else {
            // last line in the file
            return size;
        };

        if let Ok(entry) = inpt::<Entry>(line.trim()) {
            match entry {
                Entry::File(f) => size += f,
                Entry::Directory(_) => panic!("wat"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert!(matches!(inpt::<Command>("cd /"), Ok(Command::Cd(_))));
        assert!(matches!(
            inpt::<CommandLine>("$ cd /"),
            Ok(CommandLine(Command::Cd(_)))
        ));

        if let Ok(CommandLine(Command::Cd(dir))) = inpt::<CommandLine>("$ cd /") {
            assert_eq!("/", &dir);
        }
    }

    #[test]
    fn test_ls_output() {
        assert!(matches!(inpt::<Entry>("dir e"), Ok(Entry::Directory(_))));
        if let Ok(Entry::Directory(name)) = inpt::<Entry>("dir e") {
            assert_eq!("e", &name);
        }

        assert!(matches!(inpt::<Entry>("62596 h.lst"), Ok(Entry::File(_))));
        if let Ok(Entry::File(size)) = inpt::<Entry>("dir e") {
            assert_eq!(62596, size);
        }
    }
}
