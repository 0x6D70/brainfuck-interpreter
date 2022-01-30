use std::io::Read;
use std::num::Wrapping;
use std::time::Instant;

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
            memory: vec![Wrapping(0)],
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
            }

            i += 1;
        }
    }

    fn execute(&mut self) {
        while self.ins_ptr != self.instructions.len() {
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

                    while self.mem_ptr >= self.memory.len() {
                        self.memory.push(Wrapping(0));
                    }
                }
                Instruction::Left(n) => self.mem_ptr -= n,
                Instruction::Open => {
                    if self.memory[self.mem_ptr] == Wrapping(0) {
                        let mut counter = 1;

                        while self.instructions[self.ins_ptr] != Instruction::Close || counter != 0
                        {
                            self.ins_ptr += 1;

                            if self.instructions[self.ins_ptr] == Instruction::Open {
                                counter += 1;
                            } else if self.instructions[self.ins_ptr] == Instruction::Close {
                                counter -= 1;
                            }
                        }
                    }
                }
                Instruction::Close => {
                    if self.memory[self.mem_ptr] != Wrapping(0) {
                        let mut counter = 1;

                        while self.instructions[self.ins_ptr] != Instruction::Open || counter != 0 {
                            self.ins_ptr -= 1;

                            if self.instructions[self.ins_ptr] == Instruction::Open {
                                counter -= 1;
                            } else if self.instructions[self.ins_ptr] == Instruction::Close {
                                counter += 1;
                            }
                        }
                    } else {
                        self.ins_ptr += 1;
                        continue;
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

            if self.instructions[self.ins_ptr] != Instruction::Close {
                self.ins_ptr += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Inc(usize),
    Dec(usize),
    Right(usize),
    Left(usize),
    Open,
    Close,
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
            '[' => Some(Instruction::Open),
            ']' => Some(Instruction::Close),
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

    let now = Instant::now();

    bf.execute();

    let elapsed = now.elapsed();

    println!("execution took {} ms", elapsed.as_millis());
}
