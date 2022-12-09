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
    head: Coords,
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
    tail_coords.insert(world.knots[8]);

    for m in head_movement {
        println!("== {m:?} == ");
        println!();
        for _ in 0..m.steps {
            world.move_head_once(m.direction);
            world.move_knots_once();
            tail_coords.insert(world.knots[8]);
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
        for _ in 0..9 {
            knots.push(start);
        }

        World {
            width,
            height,
            start,
            head: start,
            knots,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.head {
                    print!("H");
                    continue;
                } else if let Some((i, _)) = self
                    .knots
                    .iter()
                    .enumerate()
                    .find(|&(i, knot)| (x, y) == *knot)
                {
                    print!("{}", i + 1);
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
        let (x, y) = self.head;
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
        self.head = new_pos;
    }

    fn move_knots_once(&mut self) {
        let (x, y) = self.knots[0];
        let dx = x - self.head.0;
        let dy = y - self.head.1;

        // Tail is "touching" head -- don't move:
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        let x = match dx.cmp(&0) {
            // tail is to the right; move left
            Ordering::Greater => x - 1,
            // tail is to the left; move right
            Ordering::Less => x + 1,
            Ordering::Equal => x,
        };

        let y = match dy.cmp(&0) {
            // tail is below; move up
            Ordering::Greater => y - 1,
            // tail is above; move down
            Ordering::Less => y + 1,
            Ordering::Equal => y,
        };

        self.knots[0] = (x, y);
    }
}
