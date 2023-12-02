# Advent of code 2023
This repo contain my solutions to advent of code 2023

## To run a specific solution:
```sh
$ cargo run --bin day-01-p1
```

## To create a new solution directory:
```sh
$ cargo run --bin new_challenge day-99
```

This command will create a new directory called `day-99`, containing three files
that are, `part1.rs`, `part2.rs` and `input.txt`

This command will also add two additional binary targets in the Cargo.toml
file as:
```toml
# Rest of the file

[[bin]]
name = "day-99-p1"
path = "day-99/part1.rs"

[[bin]]
name = "day-99-p2"
path = "day-99/part2.rs"
```
