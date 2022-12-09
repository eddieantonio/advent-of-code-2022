use inpt::{self, Inpt};

#[derive(Inpt, Debug, Clone, Copy)]
enum Movement {
    #[inpt(regex = r"R (\d+)")]
    Right(i32),
    #[inpt(regex = r"U (\d+)")]
    Up(i32),
    #[inpt(regex = r"D (\d+)")]
    Down(i32),
    #[inpt(regex = r"L (\d+)")]
    Left(i32),
}

type Coords = (i32, i32);

#[derive(Inpt, Debug)]
struct World {
    width: i32,
    height: i32,
    start: Coords,
    head: Coords,
    tail: Coords,
}

#[inpt::main]
fn main(head_movement: Vec<Movement>) {
    println!("{head_movement:?}");
    let mut world = World::from_bounds(&head_movement);

    println!("== Initial State ==");
    println!();
    world.print();
    println!();

    for m in head_movement {
        println!("== {m:?} == ");
        println!();
        for _ in 0..m.steps() {
            world.move_head_once(m);
            world.print();
            println!();
            world.move_tail_once();
            world.print();
            println!();
        }
    }
}

impl World {
    fn from_bounds(head_movement: &[Movement]) -> Self {
        use Movement::*;

        let mut min_width = 0;
        let mut max_width = 1;
        let mut min_height = 0;
        let mut max_height = 1;
        let mut x = 0;
        let mut y = 0;

        for m in head_movement {
            match m {
                Right(x_d) => x += x_d,
                Left(x_d) => x -= x_d,
                Up(y_d) => y -= y_d,
                Down(y_d) => y += y_d,
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

        World {
            width,
            height,
            start,
            head: start,
            tail: start,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.head {
                    print!("H");
                } else if (x, y) == self.tail {
                    print!("T");
                } else if (x, y) == self.start {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn move_head_once(&mut self, m: Movement) {
        let (x, y) = self.head;
        use Movement::*;
        let new_pos = match m {
            Right(_) => (x + 1, y),
            Up(_) => (x, y - 1),
            Down(_) => (x, y + 1),
            Left(_) => (x - 1, y),
        };
        let (x, y) = new_pos;
        assert!(x >= 0);
        assert!(x < self.width);
        assert!(y >= 0);
        assert!(y < self.height);
        self.head = new_pos;
    }

    fn move_tail_once(&mut self) {}
}

impl Movement {
    fn steps(self) -> i32 {
        use Movement::*;
        match self {
            Right(s) => s,
            Up(s) => s,
            Down(s) => s,
            Left(s) => s,
        }
    }
}
