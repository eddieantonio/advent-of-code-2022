use inpt::{inpt, Inpt};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
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

type Coords = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct NaiveEdge(Coords, Coords);

#[derive(Debug)]
struct Graph {
    width: usize,
    #[allow(unused)]
    height: usize,
    adjacency_list: Vec<Vec<Edge>>,
}

#[derive(Debug)]
struct Edge {
    destination: usize,
}

// Copied from: https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

fn main() {
    let heightmap = heightmap_from_stdin();

    let graph = build_graph(&heightmap);
    let start = find_start(&heightmap);
    let end = find_end(&heightmap);

    let s = graph.shortest_path(start, end);
    println!("{s:?}");
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

fn find_end(heightmap: &[Vec<Tile>]) -> Coords {
    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::End) {
                return (x, y);
            }
        }
    }
    panic!("end not found!");
}

fn build_graph(heightmap: &Vec<Vec<Tile>>) -> Graph {
    assert!(!heightmap.is_empty());
    let width = heightmap[0].len();
    let height = heightmap.len();

    // Figure out the edges first:
    let mut edges = HashSet::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(below) = heightmap.get_tile(x, y + 1) {
                if tile.can_climb_to(below) {
                    edges.insert(NaiveEdge((x, y), (x, y + 1)));
                }
            }

            if let Some(right) = heightmap.get_tile(x + 1, y) {
                if tile.can_climb_to(right) {
                    edges.insert(NaiveEdge((x, y), (x + 1, y)));
                }
            }
        }
    }

    let mut edges: Vec<_> = edges.into_iter().collect();
    edges.sort();
    //for edge in edges.iter() {
    //    eprintln!(" - {edge:?}");
    //}

    let mut graph = Graph {
        width,
        height,
        adjacency_list: vec![],
    };

    // Now build an adjacency list based on the edges we collected.
    // Yes, this is O(n**2) :/
    for y in 0..height {
        for x in 0..width {
            let neighbours: Vec<_> = edges
                .iter()
                .filter_map(|&NaiveEdge(a, b)| {
                    if a == (x, y) {
                        Some(b)
                    } else if b == (x, y) {
                        Some(a)
                    } else {
                        None
                    }
                })
                .map(|node| Edge {
                    destination: graph.coords_to_index(node),
                })
                .collect();

            graph.adjacency_list.push(neighbours);
        }
    }

    //for (node, neighbours) in graph.adjacency_list.iter().enumerate() {
    //    eprint!("Node {node}: ");
    //    eprintln!("{neighbours:?}");
    //}

    graph
}

impl Graph {
    fn coords_to_index(&self, (x, y): Coords) -> usize {
        x + self.width * y
    }

    fn index_to_coords(&self, i: usize) -> Coords {
        (i % self.width, i / self.width)
    }

    // This is adapated from: https://doc.rust-lang.org/std/collections/binary_heap/index.html
    fn shortest_path(&self, start: Coords, end: Coords) -> Option<usize> {
        println!("Shortest path from {start:?} to {end:?}");
        let start = self.coords_to_index(start);
        let end = self.coords_to_index(end);
        println!("(Nodes {start} to {end})");

        // Initially, distance to everything is max.
        let mut distance: Vec<usize> = (0..self.adjacency_list.len()).map(|_| usize::MAX).collect();

        let mut heap = BinaryHeap::new();

        // We start at the start!
        distance[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        // While there are nodes to visit:
        while let Some(State { cost, position }) = heap.pop() {
            println!("Considering {:?}:", self.index_to_coords(position));
            // We found the goal! We're done:
            if position == end {
                return Some(cost);
            }

            // Have we already found a better way to this node?
            if cost > distance[position] {
                continue;
            }

            // For each neighbour, see if we can find a way with a lower cost going through this
            // node:
            for edge in &self.adjacency_list[position] {
                let next = State {
                    cost: cost + 1,
                    position: edge.destination,
                };

                // Lower cost found! Add it to the frontier and continue.
                if next.cost < distance[next.position] {
                    println!(
                        " - lower cost to {:?}:",
                        self.index_to_coords(next.position)
                    );
                    heap.push(next);
                    // We have found a better way:
                    distance[next.position] = next.cost;
                }
            }
        }

        // okay, this is messed up
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.coords_to_index((x, y));
                if distance[i] < usize::MAX {
                    print!(".");
                } else {
                    print!("X");
                }
            }
            println!();
        }

        // Goal not reachable.
        None
    }
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

    fn can_climb_to(self, other: Self) -> bool {
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

// Copied from https://doc.rust-lang.org/std/collections/binary_heap/index.html
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We flip the order of costs.
        // In case of ties, we compare positions -- this step is necessary to make implementatinos
        // of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // There's a complete ordering, so just delegate:
        Some(self.cmp(other))
    }
}
