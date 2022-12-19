use std::error::Error;
use std::fs;
use std::time::{Duration, Instant};

/// For the sake of simplicity, just return puzzle answers as strings
pub type PuzzleResult = Result<String, String>;

pub struct Day {
    /// The day number (e.g., 1, 2, ..., 25)
    pub num: u8,

    /// Pointer to the function to solve part 1 of the day's puzzle
    pub p1: fn(&'static str) -> PuzzleResult,

    /// Pointer to the function to solve part 2 of the day's puzzle
    pub p2: fn(&'static str) -> PuzzleResult,

    /// Path to the file with the puzzle input
    pub input: &'static str,
}

impl Day {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let input = to_static_str(fs::read_to_string(self.input)?);

        println!("Day {}:", self.num);
        run_part("Part 1", self.p1, &input)?;
        run_part("Part 2", self.p2, &input)?;

        Ok(())
    }
}

// probably _not_ how we want to do this...
fn to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn time_str(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() - (millis * 1000);
    format!("{}s {}ms {}µs", secs, millis, micros)
}

/// The number of characters to have in the "answer box" when we print out the result.
/// If the answer is fewer than this many characters, we fill with spaces. This way,
/// we can nicely align our timing information in a column to the right of our answers.
const ANSWER_COLS: usize = 25;

fn run_part(
    title: &str,
    runner: fn(&'static str) -> PuzzleResult,
    input: &'static str,
) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let answer = runner(input)?;
    let dur = Instant::now().duration_since(start);

    // Figure out how many ' ' we need to pad the answer with to make nice columns
    // with our answers & timing info.
    let answer_chars = answer.chars().count();
    let pad_len = ANSWER_COLS - answer_chars;

    println!(
        "* {}: {}{}(took {})",
        title,
        answer,
        " ".repeat(pad_len), // make it pretty
        time_str(dur)
    );
    Ok(())
}
