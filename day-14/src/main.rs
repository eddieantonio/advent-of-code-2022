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
    #[allow(dead_code)]
    max_y: IType,
    current_sand_grain: Option<(usize, usize)>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Material {
    Air,
    Rock,
    SandAtRest,
}

const SAND_ORIGIN: Coords = (500, 0);

fn main() {
    let paths = parse_paths();

    let (mut min_x, mut min_y) = SAND_ORIGIN;
    let (mut max_x, mut max_y) = SAND_ORIGIN;

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

    let mut grains_at_rest = 0;
    while !cave.has_sand_resting_at_origin() {
        cave.add_sand(SAND_ORIGIN);
        cave.simulate_until_rest();
        grains_at_rest += 1;
    }
    println!("{grains_at_rest}");
}

#[derive(Debug)]
enum MoveInto {
    EmptySquare(GridPosition),
    RestingPosition(GridPosition),
    LeftAbyss,
    RightAbyss,
}

impl Cave {
    fn new((min_x, min_y): Coords, (max_x, max_y): Coords) -> Self {
        // Make room for the floor:
        let max_y = max_y + 2;

        let mut grid: Vec<Vec<Material>> = (min_y..=max_y)
            .map(|_| (min_x..=max_x).map(|_| Material::Air).collect())
            .collect();

        // Draw the infinite floor
        let floor = grid.iter_mut().rev().next().unwrap();
        for tile in floor.iter_mut() {
            *tile = Material::Rock;
        }

        Cave {
            min_x,
            min_y,
            max_x,
            max_y,
            grid,
            current_sand_grain: None,
        }
    }

    fn has_sand_resting_at_origin(&self) -> bool {
        let (x, y) = self.coords_to_indices(SAND_ORIGIN);
        matches!(self.grid[y][x], Material::SandAtRest)
    }

    fn simulate_until_rest(&mut self) {
        self.print_frame();
        while self.current_sand_grain.is_some() {
            self.simulate_one();
            self.print_frame();
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
            MoveInto::LeftAbyss => {
                // Have to stretch all the grids and try again.
                for row in self.grid.iter_mut() {
                    row.insert(0, Material::Air);
                }

                let floor = self.grid.iter_mut().rev().next().unwrap();
                floor[0] = Material::Rock;

                let (x, y) = self.current_sand_grain.unwrap();

                // All coordinates got shifted by one.
                self.current_sand_grain = Some((x + 1, y));
                self.min_x -= 1;
            }
            MoveInto::RightAbyss => {
                // Have to stretch all the grids and try again.
                for row in self.grid.iter_mut() {
                    row.push(Material::Air);
                }
                let floor = self.grid.iter_mut().rev().next().unwrap();
                let tile = floor.iter_mut().rev().next().unwrap();
                *tile = Material::Rock;

                self.max_x += 1;
            }
        }
    }

    fn _try_move_sand(&mut self) -> MoveInto {
        let (x, y) = self
            .current_sand_grain
            .expect("should only simulate with sand");

        assert!(y < self.grid.len() - 1, "sand below the infinite floor");

        // Look straight down
        if self.grid[y + 1][x].is_empty() {
            return MoveInto::EmptySquare((x, y + 1));
        }

        // Before we can look left, check if we'll drop into the abyss:
        if x == 0 {
            return MoveInto::LeftAbyss;
        }

        // Look one step down and to the left
        if self.grid[y + 1][x - 1].is_empty() {
            return MoveInto::EmptySquare((x - 1, y + 1));
        }

        // Before we can look right, see if it leads into the abyss:
        if x == self.grid[0].len() - 1 {
            return MoveInto::RightAbyss;
        }

        // Look one step down and to the right
        if self.grid[y + 1][x + 1].is_empty() {
            return MoveInto::EmptySquare((x + 1, y + 1));
        }

        MoveInto::RestingPosition((x, y))
    }

    #[cfg(not(feature = "animate"))]
    #[inline(always)]
    fn print_frame(&self) {
        // do nothing
    }

    #[cfg(feature = "animate")]
    fn print_frame(&self) {
        use std::{thread, time};
        print!("\x1B[2J\x1B[1;1H");
        self.print();
        // 20ms == 50 FPS
        let one_frame = time::Duration::from_millis(20);
        thread::sleep(one_frame);
    }

    #[cfg(feature = "animate")]
    fn print(&self) {
        let sand_location = self
            .current_sand_grain
            .unwrap_or((self.max_x as usize + 1, self.max_y as usize + 1));

        assert_eq!(0, self.min_y);
        for (y, row) in self.grid.iter().enumerate() {
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
    #[cfg(feature = "animate")]
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
