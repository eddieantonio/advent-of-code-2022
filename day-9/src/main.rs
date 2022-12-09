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
    let world = World::from_bounds(&head_movement);
    world.print();
}

impl World {
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
}
