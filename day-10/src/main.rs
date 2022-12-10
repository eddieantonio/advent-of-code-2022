use inpt::{self, Inpt};

#[derive(Inpt, Debug, Copy, Clone)]
enum Instruction {
    #[inpt(regex = "noop")]
    Noop,
    #[inpt(regex = r"addx\s+(-?\d+)")]
    AddX(i64),
}

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
    println!("{instructions:?}");

    let mut cpu = CPU::new(&instructions);

    while cpu.tick() {
        println!("IR: {:?}", cpu.current_instruction());
        println!("Cycles: {}", cpu.cycles_taken);
        println!("X: {}", cpu.x);

        if let Some(cycles) = cpu.signal_clock() {
            let signal_strength = cpu.x * cycles;
            println!("SIGNAL: {}", signal_strength);
        }

        println!();
    }

    println!("Cycles: {}", cpu.cycles_taken);
    println!("X: {}", cpu.x);
    println!();
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

    fn tick(&mut self) -> bool {
        use Instruction::*;

        self.cycles_taken += 1;

        if self.cycles_left_for_current_instruction > 1 {
            self.cycles_left_for_current_instruction -= 1;
            return true;
        }

        match self.current_instruction() {
            AddX(op) => {
                self.x += op;
            }
            _ => (),
        }

        self.program_counter += 1;
        if self.program_counter >= self.instructions.len() {
            return false;
        }

        self.cycles_left_for_current_instruction = self.current_instruction().cycles();

        true
    }

    fn signal_clock(&self) -> Option<i64> {
        if (self.cycles_taken + 20) % 40 == 0 {
            Some(self.cycles_taken)
        } else {
            None
        }
    }

    fn current_instruction(&self) -> Instruction {
        self.instructions[self.program_counter]
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
