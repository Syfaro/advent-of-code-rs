# advent-of-code-rs

Some work on the [Advent of Code](https://adventofcode.com/), in Rust.

Each day can be run with `cargo run --bin`. Followed by a 4 digit year, hypen,
and 2 digit day. Running a day assumes the input is provided in a file with the
same name with a txt extension in the inputs directory.

Example: `cargo run --bin 2015-01` with input in `./inputs/2015-01.txt`.

Each day also has extensive test coverage which can be run without an input file
by executing `cargo test --bin 2015-01`.

More output can be obtained by setting the `RUST_LOG` environment variable to
`debug` or `trace`.
