# A Brainfuck interpreter written in Rust

To run the mandelbrot program execute:
```bash
cargo run --release -- ./test_files/mandelbrot.b
```

Ideas:
 * optimize -> example convert ++ to Instruction::Inc(2), goal: execute mandelbrot.b in 1s
