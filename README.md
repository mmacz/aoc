# Advent of Code – Rust Solutions

![Build status](https://github.com/mmacz/aoc/actions/workflows/aoc2024.yml/badge.svg)
\
![Build status](https://github.com/mmacz/aoc/actions/workflows/aoc2019.yml/badge.svg)

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) programming puzzles, implemented in Rust.

## 📁 Repository Structure

Each year is organized into its own directory. For example:
- `2019/`
- `2024/`
- etc.

Within each year, you'll find one directory per day, typically named `dayXX`, containing the source code and input files for that day's puzzle. Inputs are not provided because of legal reasons.

## 🚀 Running Solutions

You need [Rust](https://www.rust-lang.org/) installed.

To run a specific day's solution:
```bash
cargo run --bin aoc<year> -- input<day>.txt
```

Solutions are reading input from files like `inputX.txt`. Where 'X' is the day number.

## 🛠️ Development

To run tests for a given day:
```bash
cargo test --bin aoc<year>
```

## 🌟 About Advent of Code

[Advent of Code](https://adventofcode.com/) is an annual set of programming challenges. You can try them yourself or learn from others' solutions.

## 📄 License

mhis project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

*Feel free to fork, open issues, or submit pull requests!*

