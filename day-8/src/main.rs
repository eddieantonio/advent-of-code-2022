use std::io::{self, BufRead};
use std::iter::repeat;
use std::ops::Index;

type Coords = (usize, usize);

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

fn main() {
    let trees = read_grid();

    let width = trees.width();
    let height = trees.height();

    let best_score = (1..height - 1)
        .flat_map(|y| {
            let trees = &trees;
            (1..width - 1).map(move |x| {
                let mut scenic_score = 1;
                scenic_score *= score_above(x, y, trees);
                scenic_score *= score_below(x, y, trees);
                scenic_score *= score_left(x, y, trees);
                scenic_score *= score_right(x, y, trees);
                scenic_score
            })
        })
        .max()
        .unwrap();

    println!("{best_score}");
}

fn score_above(x: usize, y: usize, grid: &Grid) -> usize {
    use std::cmp::Ordering;
    let mut score = 0;
    for (x2, y2) in grid.above((x, y)) {
        match grid[(x, y)].cmp(&grid[(x2, y2)]) {
            Ordering::Greater => {
                score += 1;
            }
            Ordering::Equal => {
                score += 1;
                break;
            }
            Ordering::Less => {
                score += 1;
                break;
            }
        }
    }
    score
}

fn score_below(x: usize, y: usize, grid: &Grid) -> usize {
    use std::cmp::Ordering;
    let mut score = 0;
    for (x2, y2) in grid.below((x, y)) {
        match grid[(x, y)].cmp(&grid[(x2, y2)]) {
            Ordering::Greater => {
                score += 1;
            }
            Ordering::Equal => {
                score += 1;
                break;
            }
            Ordering::Less => {
                score += 1;
                break;
            }
        }
    }
    score
}

fn score_left(x: usize, y: usize, grid: &Grid) -> usize {
    use std::cmp::Ordering;
    let mut score = 0;
    for (x2, y2) in grid.left((x, y)) {
        match grid[(x, y)].cmp(&grid[(x2, y2)]) {
            Ordering::Greater => {
                score += 1;
            }
            Ordering::Equal => {
                score += 1;
                break;
            }
            Ordering::Less => {
                score += 1;
                break;
            }
        }
    }
    score
}

fn score_right(x: usize, y: usize, grid: &Grid) -> usize {
    use std::cmp::Ordering;
    let mut score = 0;
    for (x2, y2) in grid.right((x, y)) {
        match grid[(x, y)].cmp(&grid[(x2, y2)]) {
            Ordering::Greater => {
                score += 1;
            }
            Ordering::Equal => {
                score += 1;
                break;
            }
            Ordering::Less => {
                score += 1;
                break;
            }
        }
    }
    score
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
