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

#[inpt::main]
fn main(head_movement: Vec<Movement>) {
    use Movement::*;
    println!("{head_movement:?}");

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
    let x = x - min_width;
    let y = y - min_height;
    println!("normalized: {x}, {y}");
}
