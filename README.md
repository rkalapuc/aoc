# Advent of Code

## Quick setup
``` shell
rustup default stable
cargo install cargo-generate
brew install just
```

## Commands
Create package for a new day:
```shell
just create <year> <day>
```

For example:
```shell
just create 2023 01
```

Execute binary for a given year, day and task:
```shell
just run <year> <day> <part>
```
where `part` is either `1` or `2`

For example:
```shell
just run 2023 01 1
```

## Events
* [[2023]](2023/README.md) **50**⭐
* [[2022]](2022/README.md) **10**⭐