# GPU||Cpu Stress Test --threaded

Testing GPU and CPU using pytorch and Rust using Threading for Absoulte performance

## Overview

This project implements an awesome stress on GPU and Cpu. It showcases some of the cool features of Rust like:

- Blazing fast performance
- Memory safety without garbage collection
- Zero-cost abstractions

## Getting Started

### Prerequisites

You'll need to have Rust installed on your system. You can install it from [https://rustup.rs](https://rustup.rs).

### Installing

Clone the project and build it with Cargo:

```
git clone https://github.com/dewasahu2003/ThreadedStressRust.git
cd ThreadedStressRust
cargo build
```

### Running

To run the program:

```
cargo run --release
```

This will build in release mode and execute the binary.

## Usage

The program allows you to do something very useful, like so:

`cargo run -- --help`:to get help and more information about commands
`cargo run cpu `:giving stress on cpu
`cargo run gpu `:giving stress on cpu
-----------------------------------------------
`cargo run t-cpu `:giving stress on cpu --threaded
`cargo run t-gpu `:giving stress on gpu --threaded 

This reads `input.txt`, performs the awesome algorithm, and writes the output to `output.txt`.

## Look at the difference
- Cpu normal
    


## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Author

Dewa Sahu - [Website](https://portfolio-beryl-seven-13.vercel.app/) / [GitHub](https://github.com/dewasahu2003)

