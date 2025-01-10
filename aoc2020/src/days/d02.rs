use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let num_valid = input
        .lines()
        .filter(|l| {
            let mut l = l.split(": ");
            let policy = l.next().unwrap();
            let password = l.next().unwrap();

            // Get the character to check in the password policy
            let p_char = policy
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            // Get the minimum number of occurrences of this character in the password
            let p_min: usize = policy
                .split_whitespace()
                .next()
                .unwrap()
                .split("-")
                .next()
                .unwrap()
                .parse()
                .unwrap();

            // Get the maximum number of occurrences of this character in the password
            let p_max: usize = policy
                .split_whitespace()
                .next()
                .unwrap()
                .split("-")
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let char_count = password.chars().filter(|&c| c == p_char).count();
            p_min <= char_count && char_count <= p_max
        })
        .count();

    Ok(format!("{num_valid}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let num_valid = input
        .lines()
        .filter(|l| {
            let mut l = l.split(": ");
            let policy = l.next().unwrap();
            let password = l.next().unwrap();

            // Get the character to check in the password policy
            let p_char = policy
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            // Get the first location to check for that char
            let loc1: usize = policy
                .split_whitespace()
                .next()
                .unwrap()
                .split("-")
                .next()
                .unwrap()
                .parse()
                .unwrap();

            // Get the second location to check for that char
            let loc2: usize = policy
                .split_whitespace()
                .next()
                .unwrap()
                .split("-")
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let loc1 = password.chars().skip(loc1 - 1).next().unwrap();
            let loc2 = password.chars().skip(loc2 - 1).next().unwrap();

            (loc1 == p_char) ^ (loc2 == p_char)
        })
        .count();

    Ok(format!("{num_valid}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "2");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "1");
    }
}
