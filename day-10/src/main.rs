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

    let mut sum_signal_strength = 0;

    loop {
        cpu.cycles_taken += 1;
        cpu.cycles_left_for_current_instruction -= 1;

        let commit = cpu.cycles_left_for_current_instruction == 0;
        println!("Starting Cycle: {}", cpu.cycles_taken);
        println!("IR: {:?}", cpu.current_instruction());
        println!("X: {}", cpu.x);

        if let Some(cycles) = cpu.signal_clock() {
            let signal_strength = cpu.x * cycles;
            println!("SIGNAL: {}", signal_strength);
            sum_signal_strength += signal_strength;
        };

        if commit {
            println!("COMMIT");
            cpu.commit_current_instruction();
            cpu.program_counter += 1;
            if let Some(op) = cpu.current_instruction() {
                cpu.cycles_left_for_current_instruction = op.cycles();
            }
        }

        println!("Ending Cycle: {}", cpu.cycles_taken);
        println!("X: {}", cpu.x);
        println!();

        if cpu.current_instruction().is_none() {
            break;
        }
    }

    println!("Cycles: {}", cpu.cycles_taken);
    println!("X: {}", cpu.x);
    println!();
    
    println!("{sum_signal_strength}");
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
        match self.current_instruction().unwrap() {
            AddX(op) => {
                self.x += op;
            }
            _ => (),
        }
    }

    fn signal_clock(&self) -> Option<i64> {
        if (self.cycles_taken + 20) % 40 == 0 {
            Some(self.cycles_taken)
        } else {
            None
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
