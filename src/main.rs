use std::io::Read;
use std::num::Wrapping;
use std::time::Instant;

#[derive(Debug)]
struct Brainfuck {
    instructions: Vec<Instruction>,
    memory: Vec<Wrapping<u8>>,
    ins_ptr: usize,
    mem_ptr: usize
}

impl Brainfuck {
    fn new(instructions: Vec<Instruction>) -> Self {
        Brainfuck {
            instructions,
            memory: vec![Wrapping(0)],
            ins_ptr: 0,
            mem_ptr: 0
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
                Instruction::Inc => self.memory[self.mem_ptr] += Wrapping(1),
                Instruction::Dec => self.memory[self.mem_ptr] -= Wrapping(1),
                Instruction::Right => {
                    self.mem_ptr += 1;
    
                    if self.mem_ptr == self.memory.len() {
                        self.memory.push(Wrapping(0));
                    }
                },
                Instruction::Left => self.mem_ptr -= 1,
                Instruction::Open => {
                    if self.memory[self.mem_ptr] == Wrapping(0) {
                        let mut counter = 1;
    
                        while self.instructions[self.ins_ptr] != Instruction::Close || counter != 0 {
                            self.ins_ptr += 1;
    
                            if self.instructions[self.ins_ptr] == Instruction::Open {
                                counter += 1;
                            } else if self.instructions[self.ins_ptr] == Instruction::Close {
                                counter -= 1;
                            }
                        }
                    }
                },
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
                },
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

#[derive(Debug,PartialEq)]
enum Instruction {
    Inc,
    Dec,
    Right,
    Left,
    Open,
    Close,
    Dot,
    Comma
}

impl Instruction {
    fn from_file(filename: &str) -> Vec<Instruction> {
        let mut instructions : Vec<Instruction> = Vec::new();

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
            '+' => Some(Instruction::Inc),
            '-' => Some(Instruction::Dec),
            '>' => Some(Instruction::Right),
            '<' => Some(Instruction::Left),
            '[' => Some(Instruction::Open),
            ']' => Some(Instruction::Close),
            '.' => Some(Instruction::Dot),
            ',' => Some(Instruction::Comma),
            _   => None,
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

    let now = Instant::now();

    bf.execute();

    let elapsed = now.elapsed();

    println!("execution took {} ms", elapsed.as_millis());
}
