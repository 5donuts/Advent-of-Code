//! Module containing a few template strings (and functions to generate those strings) to use when
//! initializing additional workspace crates for puzzle years or new day solvers.

/// Template for a generic `dXX.rs`
pub const DAY: &'static str = r##"use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    todo!("Not implemented")
}

pub fn part2(input: &str) -> PuzzleResult {
    todo!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        todo!("Not implemented");
    }

    #[test]
    fn p2() {
        todo!("Not implemented");
    }
}
"##;

/// Template for a generic `main.rs`
pub const MAIN: &'static str = r#"include!(concat!(env!("OUT_DIR"), "/days_runner.rs"));

mod days;

fn main() {
    run_days();
}
"#;

/// Template for a generic `build.rs`
pub const BUILD: &'static str = r#"use utils::generate_runner_code;

fn main() {
    generate_runner_code().unwrap();
}
"#;

/// Generate the contents of a `Cargo.toml`
pub fn cargo_toml(year: u16) -> String {
    format!(
        r#"[package]
name = "aoc{year}"
version = "0.1.0"
edition = "2021"
homepage = "https://adventofcode.com/{year}"
publish = false
exclude = ["input/"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
utils = {{ path = "../utils" }}
macros = {{ path = "../macros" }}

[build-dependencies]
utils = {{ path = "../utils" }}
"#
    )
}
