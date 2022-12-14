use inpt::{inpt, Inpt};
use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Inpt, Debug, Copy, Clone)]
enum Tile {
    #[inpt(regex = r"([a-z])")]
    Elevation(char),
    #[inpt(regex = r"S")]
    Start,
    #[inpt(regex = r"E")]
    End,
}

trait GetTile {
    fn get_tile(&self, x: usize, y: usize) -> Option<Tile>;
}

trait GetCost {
    fn get_cost(&mut self, pos: Coords) -> Option<&mut Cost>;
}

type Coords = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Edge(Coords, Coords);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cost {
    NotVisited,
    Visited(u32),
}

fn main() {
    let heightmap = heightmap_from_stdin();
    assert!(!heightmap.is_empty());
    let width = heightmap[0].len();
    let height = heightmap.len();

    let edges = build_graph(&heightmap);
    let mut edges: Vec<_> = edges.into_iter().collect();
    edges.sort();
    for edge in edges.into_iter() {
        println!(" - {edge:?}");
    }

    let start = find_start(&heightmap);
    let mut costs: Vec<Vec<Cost>> = (0..height)
        .map(|_| (0..width).map(|_| Cost::NotVisited).collect())
        .collect();

    *costs.get_cost(start).unwrap() = Cost::Visited(0);

    for (_, row) in costs.iter().enumerate() {
        for (_, cost) in row.iter().enumerate() {
            match cost {
                Cost::NotVisited => print!("∞"),
                Cost::Visited(n) => print!("{n}"),
            }
        }
        println!();
    }
}

fn heightmap_from_stdin() -> Vec<Vec<Tile>> {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| inpt(&c.to_string()).unwrap())
                .collect()
        })
        .collect()
}

fn find_start(heightmap: &[Vec<Tile>]) -> Coords {
    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Start) {
                return (x, y);
            }
        }
    }
    panic!("start not found!");
}

fn build_graph(heightmap: &Vec<Vec<Tile>>) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(below) = heightmap.get_tile(x, y + 1) {
                if tile.compatible_height(below) {
                    edges.insert(Edge((x, y), (x, y + 1)));
                }
            }

            if let Some(right) = heightmap.get_tile(x + 1, y) {
                if tile.compatible_height(right) {
                    edges.insert(Edge((x, y), (x + 1, y)));
                }
            }
        }
    }

    edges
}

impl Tile {
    fn height(self) -> i32 {
        let base: u32 = match self {
            Tile::Elevation(c) => c,
            Tile::Start => 'a',
            Tile::End => 'z',
        }
        .into();
        let normalized = base - u32::from('a');
        assert!(normalized <= 26);
        normalized as i32
    }

    fn compatible_height(self, other: Self) -> bool {
        (self.height() - other.height()).abs() <= 1
    }
}

impl GetTile for Vec<Vec<Tile>> {
    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if let Some(row) = self.get(y) {
            row.get(x).copied()
        } else {
            None
        }
    }
}

impl GetCost for Vec<Vec<Cost>> {
    fn get_cost(&mut self, (x, y): Coords) -> Option<&mut Cost> {
        if let Some(row) = self.get_mut(y) {
            if let Some(cost) = row.get_mut(x) {
                return Some(cost);
            }
        }

        None
    }
}
