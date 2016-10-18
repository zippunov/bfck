#BFCK

This is **Brainfuck** language interpreter written if **Rust** language.
This project provides binary interpreter utility and **Rust Crate** for
future use in other Rust projects.

I made it partly as a way to learn Rust, test possibility to make
native extensions for the NodeJs written in Rust and as a hat tip for
the Brainfuck creator Urban Dominik Müller.

## Features
- **Tape** has fixed classic length of 30000 cells
- **Tape** stores unsigned byte values
- **Tape** value rotates. So 255 + 1 = 0 and 0 - 1 = 255
- **Tape** address rotates. So 29999 + 1 = 0 and 0 - 1 = 29999
- interpreter compresses BF repeating operations into
  single operation
- interpreter calculates jumping addresses for correspondent **Forward**
  and **Back** commands in advance before execution.
  
## Build
- install Rust lang
- from the project root run
```
cargo build
```
- run some test bf file to verify if it is working
```
target/debug/bfck bf_src/bottles.bf
```

## Rust Crate API
_TODO_
