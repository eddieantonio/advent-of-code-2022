use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Visibility {
    Unknown,
    Visible,
    Hidden,
}

fn main() {
    use Visibility::*;
    let trees: Vec<Vec<u32>> = read_grid();

    let width = trees[0].len();
    let height = trees.len();
    let mut viz: Vec<_> = (0..height).map(|_| vec![Unknown; width]).collect();

    // Top and bottom
    for x in 0..width {
        viz[0][x] = Visible;
        viz[height - 1][x] = Visible;
    }

    // Left and right
    for row in viz.iter_mut() {
        row[0] = Visible;
        row[width - 1] = Visible;
    }

    #[allow(clippy::needless_range_loop)] // shut up, 📎
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // considering visibility for grid[x,y]
            let mut visible = false;
            visible |= check_above(x, y, &trees);
            visible |= check_below(x, y, &trees);
            visible |= check_right(x, y, &trees);
            visible |= check_left(x, y, &trees);

            viz[y][x] = if visible { Visible } else { Hidden };
        }
    }

    for (x, row) in trees.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            if viz[y][x].is_visible() {
                print!("| {tree} ");
            } else {
                print!("|   ");
            }
        }
        println!("|");
    }

    let trees_visible: usize = viz
        .iter()
        .map(|row| row.iter().filter(|tree| tree.is_visible()).count())
        .sum();

    println!("{trees_visible}");
}

type Coords = (usize, usize);

fn above((x, y): Coords) -> impl Iterator<Item = Coords> {
    std::iter::repeat(x).zip(0..y)
}

fn check_above(x: usize, y: usize, grid: &[Vec<u32>]) -> bool {
    above((x, y))
        .find(|(x2, y2)| grid[*y2][*x2] >= grid[y][x])
        .is_none()
}

fn check_below(x: usize, y: usize, grid: &[Vec<u32>]) -> bool {
    for y2 in y + 1..grid.len() {
        if grid[y2][x] >= grid[y][x] {
            return false;
        }
    }
    true
}

fn check_left(x: usize, y: usize, grid: &[Vec<u32>]) -> bool {
    for x2 in 0..x {
        if grid[y][x2] >= grid[y][x] {
            return false;
        }
    }
    true
}

fn check_right(x: usize, y: usize, grid: &[Vec<u32>]) -> bool {
    let width = grid[0].len();
    for x2 in x + 1..width {
        if grid[y][x2] >= grid[y][x] {
            return false;
        }
    }
    true
}

fn read_grid() -> Vec<Vec<u32>> {
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("line");
            let line = line.trim();
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect()
}

impl Visibility {
    fn is_visible(self) -> bool {
        matches!(self, Visibility::Visible)
    }
}
