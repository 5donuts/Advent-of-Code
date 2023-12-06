use std::error::Error;

use itertools::Itertools;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let prod: usize = parse_races(input)?
        .iter()
        .map(|r| r.winning_hold_times().count())
        .product();
    Ok(format!("{prod}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let count: usize = parse_race(input)?.winning_hold_times().count();
    Ok(format!("{count}"))
}

fn parse_races(input: &str) -> Result<Vec<Race>, Box<dyn Error>> {
    let times: Result<Vec<u64>, _> = input
        .lines()
        .next()
        .ok_or_else(|| format!("No time line in: {input}"))?
        .split(":")
        .skip(1)
        .next()
        .ok_or_else(|| format!("No times after ':' in: {input}"))?
        .split_whitespace()
        .into_iter()
        .map(|s| s.trim().parse())
        .collect();

    let distances: Result<Vec<u64>, _> = input
        .lines()
        .skip(1)
        .next()
        .ok_or_else(|| format!("No distance line in: {input}"))?
        .split(":")
        .skip(1)
        .next()
        .ok_or_else(|| format!("No distances after ':' in: {input}"))?
        .split_whitespace()
        .into_iter()
        .map(|s| s.trim().parse())
        .collect();

    Ok(times?
        .iter()
        .zip(distances?.iter())
        .map(|(&time, &dist)| Race::new(time, dist))
        .collect())
}

fn parse_race(input: &str) -> Result<Race, Box<dyn Error>> {
    let time: Result<u64, _> = input
        .lines()
        .next()
        .ok_or_else(|| format!("No time line in: {input}"))?
        .split(":")
        .skip(1)
        .next()
        .ok_or_else(|| format!("No time after ':' in: {input}"))?
        .split_whitespace()
        .join("")
        .parse();

    let distance: Result<u64, _> = input
        .lines()
        .skip(1)
        .next()
        .ok_or_else(|| format!("No distance line in: {input}"))?
        .split(":")
        .skip(1)
        .next()
        .ok_or_else(|| format!("No distance after ':' in: {input}"))?
        .split_whitespace()
        .join("")
        .parse();

    Ok(Race::new(time?, distance?))
}

#[derive(Debug, PartialEq)]
struct Race((u64, u64));

impl Race {
    fn new(time: u64, dist: u64) -> Self {
        Self((time, dist))
    }

    fn time(&self) -> u64 {
        self.0 .0
    }

    fn dist(&self) -> u64 {
        self.0 .1
    }

    /// How long can you hold down the button for the boat to travel a distance greater
    /// than the record distance for this race?
    fn winning_hold_times(&self) -> impl Iterator<Item = u64> + '_ {
        (1..self.time())
            .into_iter()
            .filter(|hold_time| (hold_time * (self.time() - hold_time)) > self.dist())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn p1() {
        let expected = 288;
        assert_eq!(
            part1(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    #[test]
    fn p2() {
        let expected = 71503;
        assert_eq!(
            part2(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    mod race {
        use itertools::Itertools;

        use super::*;

        #[test]
        fn parse() {
            let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
            let actual = parse_races(INPUT).unwrap();
            assert_eq!(actual, expected, "actual != expected");
        }

        #[test]
        fn parse_single() {
            let expected = Race::new(71530, 940200);
            let actual = parse_race(INPUT).unwrap();
            assert_eq!(actual, expected, "actual != expected");
        }

        #[test]
        fn winning_hold_times() {
            let cases = vec![
                (Race::new(7, 9), 2..=5),
                (Race::new(15, 40), 4..=11),
                (Race::new(30, 200), 11..=19),
            ];

            for (race, expected) in cases {
                assert_eq!(
                    race.winning_hold_times().collect_vec(),
                    expected.into_iter().collect_vec(),
                    "actual != expected"
                );
            }
        }
    }
}
