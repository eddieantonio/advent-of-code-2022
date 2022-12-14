use inpt::{inpt, Inpt};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

    // This is VERY much non-optimal, but it works:
    // basically, find the shortest path from all 'a' heights.
    //
    // Dijkstra actually computes the shortest path from the start to ALL nodes it can reach.
    // If I was smart, I would be able to reuse the calculation once, but alas, this is way easier
    // to program:
    let mut path_lengths = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if !matches!(tile, Tile::Start | Tile::Elevation('a')) {
                continue;
            }

            path_lengths.push(graph.shortest_path((x, y), end))
        }
    }
    let shortest = path_lengths.iter().flatten().min();
    println!("{shortest:?}");
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

    let mut graph = Graph {
        width,
        height,
        adjacency_list: vec![],
    };

    // Construct neighbours:
    for (y, row) in heightmap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let mut neighbours: Vec<_> = Vec::new();

            if y > 0 {
                if let Some(above) = heightmap.get_tile(x, y - 1) {
                    if tile.can_climb_to(above) {
                        neighbours.push(Edge {
                            destination: graph.coords_to_index((x, y - 1)),
                        });
                    }
                }
            }

            if x > 0 {
                if let Some(left) = heightmap.get_tile(x - 1, y) {
                    if tile.can_climb_to(left) {
                        neighbours.push(Edge {
                            destination: graph.coords_to_index((x - 1, y)),
                        });
                    }
                }
            }

            if let Some(right) = heightmap.get_tile(x + 1, y) {
                if tile.can_climb_to(right) {
                    neighbours.push(Edge {
                        destination: graph.coords_to_index((x + 1, y)),
                    });
                }
            }

            if let Some(bottom) = heightmap.get_tile(x, y + 1) {
                if tile.can_climb_to(bottom) {
                    neighbours.push(Edge {
                        destination: graph.coords_to_index((x, y + 1)),
                    });
                }
            }

            graph.adjacency_list.push(neighbours);
        }
    }

    graph
}

impl Graph {
    fn coords_to_index(&self, (x, y): Coords) -> usize {
        x + self.width * y
    }

    #[allow(unused)]
    fn index_to_coords(&self, i: usize) -> Coords {
        (i % self.width, i / self.width)
    }

    // This is adapated from: https://doc.rust-lang.org/std/collections/binary_heap/index.html
    fn shortest_path(&self, start: Coords, end: Coords) -> Option<usize> {
        let start = self.coords_to_index(start);
        let end = self.coords_to_index(end);

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
                    heap.push(next);
                    // We have found a better way:
                    distance[next.position] = next.cost;
                }
            }
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

    /// This is a non-associative relationship!
    /// a.can_climb_to(b) does not imply b.can_climb_to(a)!
    fn can_climb_to(self, other: Self) -> bool {
        if self.height() >= other.height() {
            // Can infinitely fall down.
            true
        } else {
            // can only step up one:
            other.height() - self.height() == 1
        }
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
