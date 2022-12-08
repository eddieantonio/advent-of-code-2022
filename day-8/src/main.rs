use std::io::{self, BufRead};
use std::iter::repeat;
use std::ops::Index;

type Coords = (usize, usize);

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

#[derive(Debug, Clone, Copy)]
enum Visibility {
    Unknown,
    Visible,
    Hidden,
}

fn main() {
    use Visibility::*;
    let trees = read_grid();

    let width = trees.width();
    let height = trees.height();
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

    for (x, row) in trees.0.iter().enumerate() {
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

fn check_above(x: usize, y: usize, grid: &Grid) -> bool {
    !grid
        .above((x, y))
        .any(|(x2, y2)| grid[(x2, y2)] >= grid[(x, y)])
}

fn check_below(x: usize, y: usize, grid: &Grid) -> bool {
    !grid
        .below((x, y))
        .any(|(x2, y2)| grid[(x2, y2)] >= grid[(x, y)])
}

fn check_left(x: usize, y: usize, grid: &Grid) -> bool {
    !grid
        .left((x, y))
        .any(|(x2, y2)| grid[(x2, y2)] >= grid[(x, y)])
}

fn check_right(x: usize, y: usize, grid: &Grid) -> bool {
    !grid
        .right((x, y))
        .any(|(x2, y2)| grid[(x2, y2)] >= grid[(x, y)])
}

fn read_grid() -> Grid {
    let stdin = io::stdin();

    Grid(
        stdin
            .lock()
            .lines()
            .map(|line| {
                let line = line.expect("line");
                let line = line.trim();
                line.chars().map(|c| c.to_digit(10).unwrap()).collect()
            })
            .collect(),
    )
}

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn above(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        repeat(x).zip(0..y)
    }

    fn below(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        repeat(x).zip(y + 1..self.height())
    }

    fn left(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        (0..x).zip(repeat(y))
    }

    fn right(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        (x + 1..self.width()).zip(repeat(y))
    }
}

impl Index<Coords> for Grid {
    type Output = u32;
    fn index(&self, (x, y): Coords) -> &Self::Output {
        &self.0[y][x]
    }
}

impl Visibility {
    fn is_visible(self) -> bool {
        matches!(self, Visibility::Visible)
    }
}
