use inpt::inpt;
use std::cmp;
use std::io::{self, BufRead};

type IType = i64;
type GridPosition = (usize, usize);
type Coords = (IType, IType);
type LineSegment = (Coords, Coords);

struct Cave {
    grid: Vec<Vec<Material>>,
    min_x: IType,
    min_y: IType,
    max_x: IType,
    max_y: IType,
    current_sand_grain: Option<(usize, usize)>,
    has_moved_sand_into_the_abyss: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Material {
    Air,
    Rock,
    SandAtRest,
}

fn main() {
    let paths = parse_paths();

    let sand_origin = (500, 0);
    let (mut min_x, mut min_y) = sand_origin;
    let (mut max_x, mut max_y) = sand_origin;

    for path in paths.iter() {
        for &((x0, y0), (x1, y1)) in path {
            min_x = cmp::min(min_x, cmp::min(x0, x1));
            max_x = cmp::max(max_x, cmp::max(x0, x1));
            min_y = cmp::min(min_y, cmp::min(y0, y1));
            max_y = cmp::max(max_y, cmp::max(y0, y1));
        }
    }

    let mut cave = Cave::new((min_x, min_y), (max_x, max_y));
    cave.add_rocks(&paths);

    let mut grains_dropped = 0;
    loop {
        println!("== adding sand ==");
        cave.add_sand(sand_origin);
        cave.print();

        cave.simulate_until_rest();
        cave.print();
        println!();

        if cave.has_moved_sand_into_the_abyss {
            break;
        } else {
            grains_dropped += 1;
        }
    }
    println!("{grains_dropped}");
}

#[derive(Debug)]
enum MoveInto {
    EmptySquare(GridPosition),
    RestingPosition(GridPosition),
    Abyss,
}

impl Cave {
    fn new((min_x, min_y): Coords, (max_x, max_y): Coords) -> Self {
        let grid = (min_y..=max_y)
            .map(|_| (min_x..=max_x).map(|_| Material::Air).collect())
            .collect();

        Cave {
            min_x,
            min_y,
            max_x,
            max_y,
            grid,
            current_sand_grain: None,
            has_moved_sand_into_the_abyss: false,
        }
    }

    fn simulate_until_rest(&mut self) {
        while self.current_sand_grain.is_some() {
            self.simulate_one()
        }
    }

    fn simulate_one(&mut self) {
        match self._try_move_sand() {
            MoveInto::EmptySquare((x, y)) => {
                self.current_sand_grain = Some((x, y));
            }
            MoveInto::RestingPosition((x, y)) => {
                self.current_sand_grain = None;
                self.grid[y][x] = Material::SandAtRest;
            }
            MoveInto::Abyss => {
                self.current_sand_grain = None;
                self.has_moved_sand_into_the_abyss = true;
            }
        }
    }

    fn _try_move_sand(&mut self) -> MoveInto {
        let (x, y) = self
            .current_sand_grain
            .expect("should only simulate with sand");

        // Check if we'll drop straight into the abyss:
        if y == self.grid.len() - 1 {
            return MoveInto::Abyss;
        }

        // Look straight down
        if self.grid[y + 1][x].is_empty() {
            return MoveInto::EmptySquare((x, y + 1));
        }

        // Before we can look left, check if we'll drop into the abyss:
        if x == 0 {
            return MoveInto::Abyss;
        }

        // Look one step down and to the left
        if self.grid[y + 1][x - 1].is_empty() {
            return MoveInto::EmptySquare((x - 1, y + 1));
        }

        // Before we can look right, see if it leads into the abyss:
        if x == self.grid[0].len() - 1 {
            return MoveInto::Abyss;
        }

        // Look one step down and to the right
        if self.grid[y + 1][x + 1].is_empty() {
            return MoveInto::EmptySquare((x + 1, y + 1));
        }

        MoveInto::RestingPosition((x, y))
    }

    fn print(&self) {
        let sand_location = self
            .current_sand_grain
            .unwrap_or((self.max_x as usize + 1, self.max_y as usize + 1));

        assert_eq!(0, self.min_y);
        for (y, row) in self.grid.iter().enumerate() {
            print!("{y} ");
            for (x, tile) in row.iter().enumerate() {
                if (x, y) == sand_location {
                    assert_eq!(*tile, Material::Air);
                    print!("+");
                } else {
                    print!("{}", tile.ascii_art());
                }
            }
            println!();
        }
    }

    fn add_rocks(&mut self, paths: &[Vec<LineSegment>]) {
        for path in paths {
            for line in path {
                self.fill_line_segment(*line);
            }
        }
    }

    fn add_sand(&mut self, location: Coords) {
        assert!(self.current_sand_grain.is_none());
        let (x, y) = self.coords_to_indices(location);
        assert_eq!(self.grid[y][x], Material::Air);
        self.current_sand_grain = Some((x, y));
    }

    fn fill_line_segment(&mut self, line: LineSegment) {
        let ((x0, y0), (x1, y1)) = self.line_segment_to_indices(line);

        if x0 == x1 {
            // vertical line
            for y in ordered(y0, y1) {
                self.grid[y][x0] = Material::Rock;
            }
        } else if y0 == y1 {
            // horizontal line
            for x in ordered(x0, x1) {
                self.grid[y0][x] = Material::Rock;
            }
        } else {
            unimplemented!("diagonal line: {line:?}");
        }
    }

    fn line_segment_to_indices(&self, (p1, p2): LineSegment) -> ((usize, usize), (usize, usize)) {
        (self.coords_to_indices(p1), self.coords_to_indices(p2))
    }

    fn coords_to_indices(&self, (x, y): Coords) -> (usize, usize) {
        ((x - self.min_x) as usize, (y - self.min_y) as usize)
    }
}

fn ordered<Idx>(a: Idx, b: Idx) -> std::ops::RangeInclusive<Idx>
where
    Idx: Ord + Copy,
{
    cmp::min(a, b)..=cmp::max(a, b)
}

fn parse_paths() -> Vec<Vec<LineSegment>> {
    let stdin = io::stdin();
    let mut paths = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut path = Vec::new();
        let mut last_coord = None;
        for part in line.split("->") {
            let current = inpt::<Coords>(part.trim()).unwrap();

            match last_coord {
                Some(previous) => {
                    path.push((previous, current));
                    last_coord = Some(current);
                }
                None => {
                    last_coord = Some(current);
                }
            }
        }

        paths.push(path);
    }

    paths
}

impl Material {
    fn ascii_art(self) -> char {
        use Material::*;
        match self {
            Air => '.',
            Rock => '#',
            SandAtRest => 'o',
        }
    }

    fn is_empty(self) -> bool {
        matches!(self, Material::Air)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}
