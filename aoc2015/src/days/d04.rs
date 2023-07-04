use md5::{Digest, Md5};
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let secret_key = input.trim();
    let mut hasher = Md5::new();
    let mut answer: u32 = 0;

    loop {
        hasher.update(format!("{}{}", secret_key, answer));
        let hash = hasher.finalize_reset();
        // Check for at least 5 leading zeroes in the hexadecimal output
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return Ok(format!("{}", answer));
        }
        answer += 1;
    }
}

pub fn part2(input: &str) -> PuzzleResult {
    let secret_key = input.trim();
    let mut hasher = Md5::new();
    let mut answer: u32 = 0;

    loop {
        hasher.update(format!("{}{}", secret_key, answer));
        let hash = hasher.finalize_reset();
        // Check for at least 6 leading zeroes in the hexadecimal output
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return Ok(format!("{}", answer));
        }
        answer += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![("abcdef", 609043), ("pqrstuv", 1048970)];
        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }
}
