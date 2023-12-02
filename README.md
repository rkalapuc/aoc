# Advent of Code 2023

## Quick setup
``` shell
rustup default stable
cargo install cargo-nextest cargo-generate
brew install just
```

## Some commands
Create package for a new day:
```shell
just create <day>
```

Execute binary for given day and task
```shell
just run <day> <part>
```
where `part` is either `part01` or `part02`

## Packages
* [Day 1](day-01)
* [Day 2](day-02)