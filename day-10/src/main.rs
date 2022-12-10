use inpt::{self, Inpt};

#[derive(Inpt, Debug, Copy, Clone)]
enum Instruction {
    #[inpt(regex = "noop")]
    Noop,
    #[inpt(regex = r"addx\s+(-?\d+)")]
    AddX(i64),
}

const CRT_WIDTH: i64 = 40;

#[derive(Debug)]
struct CPU<'a> {
    x: i64,
    cycles_taken: i64,
    program_counter: usize,
    cycles_left_for_current_instruction: usize,
    instructions: &'a [Instruction],
}

#[inpt::main]
fn main(instructions: Vec<Instruction>) {
    let mut cpu = CPU::new(&instructions);

    loop {
        cpu.cycles_taken += 1;
        cpu.cycles_left_for_current_instruction -= 1;

        let current_pixel = (cpu.cycles_taken - 1) % CRT_WIDTH;
        let sprite = cpu.x - 1..=cpu.x + 1;

        if sprite.contains(&current_pixel) {
            print!("#");
        } else {
            print!(".");
        }

        if current_pixel == CRT_WIDTH - 1 {
            println!();
        }

        let commit = cpu.cycles_left_for_current_instruction == 0;
        if commit {
            //println!("COMMIT");
            cpu.commit_current_instruction();
            cpu.program_counter += 1;
            if let Some(op) = cpu.current_instruction() {
                cpu.cycles_left_for_current_instruction = op.cycles();
            }
        }

        if cpu.current_instruction().is_none() {
            break;
        }
    }
}

impl<'a> CPU<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        let first = instructions[0];

        CPU {
            x: 1,
            cycles_taken: 0,
            program_counter: 0,
            instructions,
            cycles_left_for_current_instruction: first.cycles(),
        }
    }

    fn commit_current_instruction(&mut self) {
        use Instruction::*;
        if let Some(AddX(op)) = self.current_instruction() {
            self.x += op;
        }
    }

    fn current_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.program_counter).copied()
    }
}

impl Instruction {
    fn cycles(self) -> usize {
        use Instruction::*;
        match self {
            AddX(_) => 2,
            Noop => 1,
        }
    }
}
