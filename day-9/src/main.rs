use inpt::{self, Inpt};
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Inpt, Debug, Clone, Copy)]
#[inpt(regex = r"(\w+) (\d+)")]
struct Movement {
    direction: Direction,
    steps: i32,
}

#[derive(Inpt, Debug, Clone, Copy)]
enum Direction {
    #[inpt(regex = r"R")]
    Right,
    #[inpt(regex = r"U")]
    Up,
    #[inpt(regex = r"D")]
    Down,
    #[inpt(regex = r"L")]
    Left,
}

type Coords = (i32, i32);

#[derive(Inpt, Debug)]
struct World {
    width: i32,
    height: i32,
    start: Coords,
    knots: Vec<Coords>,
}

#[inpt::main]
fn main(head_movement: Vec<Movement>) {
    let mut world = World::from_bounds(&head_movement);

    println!("== Initial State ==");
    println!();
    world.print();
    println!();

    let mut tail_coords: HashSet<_> = HashSet::new();
    tail_coords.insert(world.knots[9]);

    for m in head_movement {
        println!("== {m:?} == ");
        println!();
        for _ in 0..m.steps {
            world.move_head_once(m.direction);
            world.move_knots_once();
            tail_coords.insert(world.knots[9]);
            world.print();
            println!();
        }
    }

    println!("Positions: {}", tail_coords.len());
}

impl World {
    fn from_bounds(head_movement: &[Movement]) -> Self {
        use Direction::*;

        let mut min_width = 0;
        let mut max_width = 1;
        let mut min_height = 0;
        let mut max_height = 1;
        let mut x = 0;
        let mut y = 0;

        for m in head_movement {
            let d = m.steps;
            match m.direction {
                Right => x += d,
                Left => x -= d,
                Up => y -= d,
                Down => y += d,
            }

            if x >= max_width {
                max_width = x + 1;
            }
            if x < min_width {
                min_width = x;
            }
            if y >= max_height {
                max_height = y + 1;
            }

            if y < min_height {
                min_height = y;
            }
        }

        let width = max_width - min_width;
        let height = max_height - min_height;
        println!("width: {width}");
        println!("height: {height}");
        println!("last pos: {x}, {y}");
        println!("{min_width}--{max_width}");
        println!("{min_height}--{max_height}");
        let x = x - min_width;
        let y = y - min_height;
        println!("normalized: {x}, {y}");
        let start = (0 - min_width, 0 - min_height);
        println!("start: {start:?}");

        let mut knots = Vec::new();
        for _ in 0..10 {
            knots.push(start);
        }

        World {
            width,
            height,
            start,
            knots,
        }
    }

    fn head(&self) -> Coords {
        self.knots[0]
    }

    fn set_head_position(&mut self, new_position: Coords) {
        self.knots[0] = new_position
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.head() {
                    print!("H");
                    continue;
                } else if let Some((i, _)) = self
                    .knots
                    .iter()
                    .enumerate()
                    .find(|&(_, knot)| (x, y) == *knot)
                {
                    // knots are one-indexed, weirdly enough.
                    print!("{i}");
                } else if (x, y) == self.start {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn move_head_once(&mut self, dir: Direction) {
        let (x, y) = self.head();
        use Direction::*;
        let new_pos = match dir {
            Right => (x + 1, y),
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
        };
        let (x, y) = new_pos;
        assert!(x >= 0);
        assert!(x < self.width);
        assert!(y >= 0);
        assert!(y < self.height);
        self.set_head_position(new_pos);
    }

    fn move_knots_once(&mut self) {
        for knot in 1..10 {
            self.move_knot_once(knot);
        }
    }

    fn move_knot_once(&mut self, which: usize) {
        assert!(which >= 1);
        assert!(which < 10);
        let (x, y) = self.knots[which];
        let (other_x, other_y) = self.knots[which - 1];
        let dx = x - other_x;
        let dy = y - other_y;

        // Knot is "touching" other knot -- don't move:
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        let x = match dx.cmp(&0) {
            // this knot is to the right; move left
            Ordering::Greater => x - 1,
            // this know is to the left; move right
            Ordering::Less => x + 1,
            Ordering::Equal => x,
        };

        let y = match dy.cmp(&0) {
            // this knot is below; move up
            Ordering::Greater => y - 1,
            // this knot is above; move down
            Ordering::Less => y + 1,
            Ordering::Equal => y,
        };

        self.knots[which] = (x, y);
    }
}
