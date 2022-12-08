use std::io::{self, BufRead};
use std::iter::repeat;
use std::ops::Index;

type Coords = (usize, usize);

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

macro_rules! score {
    ($dir: tt, $grid: ident, $x: ident, $y: ident) => {{
        let mut score = 0;
        for (x2, y2) in $grid.$dir(($x, $y)) {
            score += 1;
            if $grid[($x, $y)] <= $grid[(x2, y2)] {
                break;
            }
        }
        score
    }};
}

fn main() {
    let trees = read_grid();

    let width = trees.width();
    let height = trees.height();

    let best_score = (1..height - 1)
        .flat_map(|y| {
            // borrow `trees` here so that the closure doesn't try to move it...
            // otherwise, we'd have to borrow `y`, and that's just silly.
            let trees = &trees;
            (1..width - 1).map(move |x| {
                let mut scenic_score = 1;
                scenic_score *= score!(above, trees, x, y);
                scenic_score *= score!(below, trees, x, y);
                scenic_score *= score!(left, trees, x, y);
                scenic_score *= score!(right, trees, x, y);
                scenic_score
            })
        })
        .max()
        .unwrap();

    println!("{best_score}");
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
        repeat(x).zip((0..y).rev())
    }

    fn below(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        repeat(x).zip(y + 1..self.height())
    }

    fn left(&self, (x, y): Coords) -> impl Iterator<Item = Coords> {
        (0..x).rev().zip(repeat(y))
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
