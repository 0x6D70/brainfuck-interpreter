use std::io::Read;
use std::num::Wrapping;
use std::time::Instant;

fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

#[derive(Debug)]
struct Brainfuck {
    instructions: Vec<Instruction>,
    memory: Vec<Wrapping<u8>>,
    ins_ptr: usize,
    mem_ptr: usize,
}

impl Brainfuck {
    fn new(instructions: Vec<Instruction>) -> Self {
        Brainfuck {
            instructions,
            memory: vec![Wrapping(0); 30000],
            ins_ptr: 0,
            mem_ptr: 0,
        }
    }

    fn optimize(&mut self) {
        let mut i = 0;

        // convert multiple statements of the same type into one
        while i < self.instructions.len() - 1 {
            if let (Instruction::Inc(n1), Instruction::Inc(n2)) =
                (&self.instructions[i], &self.instructions[i + 1])
            {
                self.instructions[i] = Instruction::Inc(n1 + n2);
                self.instructions.remove(i + 1);
            } else if let (Instruction::Dec(n1), Instruction::Dec(n2)) =
                (&self.instructions[i], &self.instructions[i + 1])
            {
                self.instructions[i] = Instruction::Dec(n1 + n2);
                self.instructions.remove(i + 1);
            } else if let (Instruction::Right(n1), Instruction::Right(n2)) =
                (&self.instructions[i], &self.instructions[i + 1])
            {
                self.instructions[i] = Instruction::Right(n1 + n2);
                self.instructions.remove(i + 1);
            } else if let (Instruction::Left(n1), Instruction::Left(n2)) =
                (&self.instructions[i], &self.instructions[i + 1])
            {
                self.instructions[i] = Instruction::Left(n1 + n2);
                self.instructions.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn set_matching_paren(&mut self) {
        // set matching paren of close and open instructions

        let mut i = 0;
        while i < self.instructions.len() {
            if matches!(self.instructions[i], Instruction::Open(_)) {
                let mut counter = 1;
                let mut index = i;

                while !matches!(self.instructions[index], Instruction::Close(_)) || counter != 0 {
                    index += 1;

                    if matches!(self.instructions[index], Instruction::Open(_)) {
                        counter += 1;
                    } else if matches!(self.instructions[index], Instruction::Close(_)) {
                        counter -= 1;
                    }
                }

                self.instructions[i] = Instruction::Open(index);
            } else if matches!(self.instructions[i], Instruction::Close(_)) {
                let mut counter = 1;
                let mut index = i;

                while !matches!(self.instructions[index], Instruction::Open(_)) || counter != 0 {
                    index -= 1;

                    if matches!(self.instructions[index], Instruction::Open(_)) {
                        counter -= 1;
                    } else if matches!(self.instructions[index], Instruction::Close(_)) {
                        counter += 1;
                    }
                }

                self.instructions[i] = Instruction::Close(index);
            }

            i += 1;
        }
    }

    fn execute(&mut self) {
        let mut ins_count = 0_usize;

        let cycle_start = rdtsc();

        while self.ins_ptr != self.instructions.len() {
            ins_count += 1;
            // step through every instruction and print self
            // println!("======================================================");
            // println!("{:?}", self.instructions[self.ins_ptr]);
            // println!("{:?}", self);
            // println!("======================================================");
            // let _ = std::io::stdin().bytes().next();

            match self.instructions[self.ins_ptr] {
                Instruction::Inc(n) => self.memory[self.mem_ptr] += Wrapping(n as u8),
                Instruction::Dec(n) => self.memory[self.mem_ptr] -= Wrapping(n as u8),
                Instruction::Right(n) => {
                    self.mem_ptr += n;

                    if self.mem_ptr > self.memory.len() {
                        self.memory.resize(self.mem_ptr + 1024, Wrapping(0));
                    }
                }
                Instruction::Left(n) => self.mem_ptr -= n,
                Instruction::Open(index) => {
                    if self.memory[self.mem_ptr] == Wrapping(0) {
                        self.ins_ptr = index;
                    }
                }
                Instruction::Close(index) => {
                    if self.memory[self.mem_ptr] != Wrapping(0) {
                        self.ins_ptr = index;
                    }
                }
                Instruction::Dot => print!("{}", self.memory[self.mem_ptr].0 as char),
                Instruction::Comma => {
                    self.memory[self.mem_ptr] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(Wrapping::<u8>)
                        .unwrap();
                }
            };

            self.ins_ptr += 1;
        }

        let cycle_end = rdtsc();

        println!("number of instructions executed: {}", ins_count);
        println!("number of cycles: {}", (cycle_end - cycle_start));
        println!(
            "{} cycles / instruction",
            (cycle_end - cycle_start) as f64 / (ins_count as f64)
        );
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Inc(u8),
    Dec(u8),
    Right(usize),
    Left(usize),
    Open(usize),
    Close(usize),
    Dot,
    Comma,
}

impl Instruction {
    fn from_file(filename: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();

        let content = std::fs::read_to_string(filename).expect("error while reading file");

        for c in content.chars() {
            if let Some(ins) = Instruction::get_instruction(c) {
                instructions.push(ins);
            }
        }

        instructions
    }

    fn get_instruction(c: char) -> Option<Instruction> {
        match c {
            '+' => Some(Instruction::Inc(1)),
            '-' => Some(Instruction::Dec(1)),
            '>' => Some(Instruction::Right(1)),
            '<' => Some(Instruction::Left(1)),
            '[' => Some(Instruction::Open(0)),
            ']' => Some(Instruction::Close(0)),
            '.' => Some(Instruction::Dot),
            ',' => Some(Instruction::Comma),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: brainfuck-interpreter <filename>");
        return;
    }

    let instructions = Instruction::from_file(&args[1]);

    let mut bf = Brainfuck::new(instructions);
    bf.optimize();
    bf.set_matching_paren();

    let now = Instant::now();

    bf.execute();

    let elapsed = now.elapsed();

    println!("execution took {} ms", elapsed.as_millis());
}
