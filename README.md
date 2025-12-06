# Advent of Code Solutions

Solutions for [Advent of Code](https://adventofcode.com) puzzles, written in Rust.

## Project Structure

The repository is organized by year and day:

```
advent-of-code/
├── 2024/
│   ├── aoc-01/
│   ├── aoc-02/
│   └── ...
├── 2025/
│   ├── aoc-01/
│   ├── aoc-02/
│   └── ...
└── template/
    └── src/bin/
        ├── one.rs    # Part 1 template
        └── two.rs    # Part 2 template
```

Each day's solution is a separate Rust crate with:
- `src/bin/one.rs` - Solution for part 1
- `src/bin/two.rs` - Solution for part 2
- `inputs/example.txt` - Example input from the problem
- `inputs/real.txt` - Real puzzle input

## Running Solutions

To run a solution, navigate to the day's directory and use `cargo run`:

```bash
cd 2025/aoc-01
cargo run --bin one    # Run part 1
cargo run --bin two    # Run part 2
```

## Testing

Each solution includes tests for the example input:

```bash
cargo test
```

## Creating a New Solution

1. Copy the `template` directory to create a new day:
   ```bash
   cp -r template 2025/aoc-03
   ```

2. Update the `Cargo.toml` with the correct package name:
   ```toml
   [package]
   name = "aoc-03"
   ```

3. Add your input files:
   - `inputs/example.txt` - Example input from the problem
   - `inputs/real.txt` - Your puzzle input

4. Implement your solution in `src/bin/one.rs` and `src/bin/two.rs`

5. Update the test assertion in each file to match the expected example output# advent-of-code
