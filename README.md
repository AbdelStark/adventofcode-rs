<div align="center">
  <h1>adventofcode-rs</h1>
  <img src="docs/images/logo.png">
  <a href="#about"><strong>Advent of Code 2022 solutions in Rust 🦀</strong></a>
</div>

## About

This repository contains my solutions to the [Advent of Code 2022](https://adventofcode.com/2022) puzzles. I'm using this as an opportunity to learn & practise Rust, so the solutions are not necessarily the most efficient or idiomatic.

## Usage

To run a solution, use the following command:

```bash
cargo run --release --bin dayXX-Y
```

where `XX` is the day number and `Y` is the part number.

For example, to run the solution for day 1 part 2, use the following command:

```bash
cargo run --release --bin day01-2
```

You can configure the log level by setting the `RUST_LOG` environment variable. For example, to run the solution for day 1 part 2 with debug logging, use the following command:

```bash
RUST_LOG=debug cargo run --release --bin day01-2
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.