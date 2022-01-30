use std::io::Read;
use std::num::Wrapping;

#[derive(Debug)]
struct BrainfuckState {
    instructions: Vec<Instruction>,
    memory: Vec<Wrapping<u8>>,
    ins_ptr: usize,
    mem_ptr: usize
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: brainfuck-interpreter <filename>");
        return;
    }

    let mut instructions : Vec<Instruction> = Vec::new();

    let content = std::fs::read_to_string(&args[1]).expect("error while reading file");

    for c in content.chars() {
        if let Some(ins) = get_instruction(c) {
            instructions.push(ins);
        }
    }

    let mut state = BrainfuckState {
        instructions,
        memory: vec![Wrapping(0)],
        ins_ptr: 0,
        mem_ptr: 0
    };

    while state.ins_ptr != state.instructions.len() {

        // step through every instruction and print state
        // println!("======================================================");
        // println!("{:?}", state.instructions[state.ins_ptr]);
        // println!("{:?}", state);
        // println!("======================================================");
        // let _ = std::io::stdin().bytes().next();
        
        match state.instructions[state.ins_ptr] {
            Instruction::Inc => state.memory[state.mem_ptr] += Wrapping(1),
            Instruction::Dec => state.memory[state.mem_ptr] -= Wrapping(1),
            Instruction::Right => {
                state.mem_ptr += 1;

                if state.mem_ptr == state.memory.len() {
                    state.memory.push(Wrapping(0));
                }
            },
            Instruction::Left => state.mem_ptr -= 1,
            Instruction::Open => {
                if state.memory[state.mem_ptr] == Wrapping(0) {
                    let mut counter = 1;

                    while state.instructions[state.ins_ptr] != Instruction::Close || counter != 0 {
                        state.ins_ptr += 1;

                        if state.instructions[state.ins_ptr] == Instruction::Open {
                            counter += 1;
                        } else if state.instructions[state.ins_ptr] == Instruction::Close {
                            counter -= 1;
                        }
                    }
                }
            },
            Instruction::Close => {
                if state.memory[state.mem_ptr] != Wrapping(0) {
                    let mut counter = 1;

                    while state.instructions[state.ins_ptr] != Instruction::Open || counter != 0 {
                        state.ins_ptr -= 1;

                        if state.instructions[state.ins_ptr] == Instruction::Open {
                            counter -= 1;
                        } else if state.instructions[state.ins_ptr] == Instruction::Close {
                            counter += 1;
                        }
                    }
                } else {
                    state.ins_ptr += 1;
                    continue;
                } 
            },
            Instruction::Dot => print!("{}", state.memory[state.mem_ptr].0 as char),
            Instruction::Comma => {
                state.memory[state.mem_ptr] = std::io::stdin()
                                                        .bytes() 
                                                        .next()
                                                        .and_then(|result| result.ok())
                                                        .map(Wrapping::<u8>)
                                                        .unwrap();
            }
        }; 

        if state.instructions[state.ins_ptr] != Instruction::Close {
            state.ins_ptr += 1;
        }
    }
}
