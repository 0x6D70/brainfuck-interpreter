# A Brainfuck interpreter written in Rust

### Building and Running
For building you simply need to run
```
cargo build
```

Running a program requires the following command
```bash
cargo run --release -- /path/to/file.b
```

In `test_files` some test programs are already provided

### Ideas:
 - optimize 
   - convert ++ to Instruction::Inc(2)

### Goals
 -  execute mandelbrot.b in 1s
