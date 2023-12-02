use std::error::Error;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let bag = Set::new(12, 13, 14);

    let possible_game_ids_sum: u32 = input
        .lines()
        .map(|l| Game::try_from(l))
        .collect::<Result<Vec<Game>, _>>()?
        .iter()
        .filter_map(|g| if g.possible(&bag) { Some(g.id) } else { None })
        .sum();

    Ok(format!("{possible_game_ids_sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let total_power: u32 = input
        .lines()
        .map(|l| Game::try_from(l))
        .collect::<Result<Vec<Game>, _>>()?
        .iter()
        .map(|g| g.min_set().power())
        .sum();

    Ok(format!("{total_power}"))
}

/// A single game of one or more [`Set`]s of red, green, & blue cubes
/// pulled from a bag.
#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    /// Given a set of red, green, & blue cubes in a bag, would it be possible
    /// to have this particular game?
    fn possible(&self, bag: &Set) -> bool {
        // For each color, did we see a number of cubes greater than the number
        // in the bag during at least one set in the game?
        let red = self.sets.iter().filter(|set| set.red() > bag.red()).count() > 0;
        let green = self
            .sets
            .iter()
            .filter(|set| set.green() > bag.green())
            .count()
            > 0;
        let blue = self
            .sets
            .iter()
            .filter(|set| set.blue() > bag.blue())
            .count()
            > 0;

        // Invert this check to determine if the game was possible
        !(red || green || blue)
    }

    /// What is the minimum set of cubes required for this game to be possible?
    fn min_set(&self) -> Set {
        let red = self.sets.iter().map(|set| set.red()).max().unwrap();
        let blue = self.sets.iter().map(|set| set.blue()).max().unwrap();
        let green = self.sets.iter().map(|set| set.green()).max().unwrap();

        Set::new(red, green, blue)
    }
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;

    /// Sample input:
    /// `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = value
            .split(":")
            .next()
            .ok_or::<Box<dyn Error>>(format!("No game component in: {value}").into())?
            .trim()
            .split(" ")
            .skip(1)
            .next()
            .ok_or::<Box<dyn Error>>(format!("No game id in: {value}").into())?
            .parse()?;

        let sets: Result<Vec<Set>, _> = value
            .split(":")
            .skip(1)
            .next()
            .ok_or::<Box<dyn Error>>(format!("No sets in: {value}").into())?
            .split(";")
            .map(|s| Set::try_from(s))
            .collect();
        let sets = sets?;

        Ok(Self { id, sets })
    }
}

/// A single set of cubes pulled from a bag in a [`Game`]
#[derive(Debug, PartialEq)]
struct Set((u32, u32, u32));

impl Set {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Self((r, g, b))
    }

    fn red(&self) -> u32 {
        self.0 .0
    }

    fn green(&self) -> u32 {
        self.0 .1
    }

    fn blue(&self) -> u32 {
        self.0 .2
    }

    fn power(&self) -> u32 {
        self.red() * self.green() * self.blue()
    }
}

impl TryFrom<&str> for Set {
    type Error = Box<dyn Error>;

    /// If this set is part of a [`Game`] represented by:
    /// `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`,
    /// then the input to this function could be:
    /// * ` 3 blue, 4 red`,
    /// * ` 1 red, 2 green, 6 blue`, or
    /// * ` 2 green`
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in value.split(",") {
            let mut parts = part.trim().split(" ");
            let count = parts
                .next()
                .ok_or::<Box<dyn Error>>(format!("No numeric component in: {part}").into())?
                .parse()?;
            let color = parts
                .next()
                .ok_or::<Box<dyn Error>>(format!("No color component in line: {part}").into())?;

            match color {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                _ => return Err(format!("Invalid color component {color}").into()),
            }
        }

        Ok(Self((red, green, blue)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod game {
        use super::*;

        #[test]
        fn parse() {
            let cases = vec![
                (
                    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                    Game {
                        id: 1,
                        sets: vec![Set::new(4, 0, 3), Set::new(1, 2, 6), Set::new(0, 2, 0)],
                    },
                ),
                (
                    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                    Game {
                        id: 2,
                        sets: vec![Set::new(0, 2, 1), Set::new(1, 3, 4), Set::new(0, 1, 1)],
                    },
                ),
                (
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                    Game {
                        id: 3,
                        sets: vec![Set::new(20, 8, 6), Set::new(4, 13, 5), Set::new(1, 5, 0)],
                    },
                ),
                (
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                    Game {
                        id: 4,
                        sets: vec![Set::new(3, 1, 6), Set::new(6, 3, 0), Set::new(14, 3, 15)],
                    },
                ),
                (
                    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                    Game {
                        id: 5,
                        sets: vec![Set::new(6, 3, 1), Set::new(1, 2, 2)],
                    },
                ),
            ];

            for (input, expected) in cases {
                assert_eq!(
                    Game::try_from(input).unwrap(),
                    expected,
                    "Game {}",
                    expected.id
                );
            }
        }

        #[test]
        fn possible() {
            let bag = Set::new(12, 13, 14);
            let cases =
                vec![
            (
                Game::try_from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
                true,
            ),
            (
                Game::try_from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                    .unwrap(),
                true,
            ),
            (
                Game::try_from(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                )
                .unwrap(),
                false,
            ),
            (
                Game::try_from(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                )
                .unwrap(),
                false,
            ),
            (
                Game::try_from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
                true,
            ),
        ];

            for (input, expected) in cases {
                assert_eq!(input.possible(&bag), expected, "Game {}", input.id);
            }
        }

        #[test]
        fn fewest_cubes() {
            let cases = vec![
                (
                    Game::try_from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                        .unwrap(),
                    Set::new(4, 2, 6),
                ),
                (
                    Game::try_from(
                        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                    )
                    .unwrap(),
                    Set::new(1, 3, 4),
                ),
                (
                    Game::try_from(
                        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                    )
                    .unwrap(),
                    Set::new(20, 13, 6),
                ),
                (
                    Game::try_from(
                        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                    )
                    .unwrap(),
                    Set::new(14, 3, 15),
                ),
                (
                    Game::try_from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
                        .unwrap(),
                    Set::new(6, 3, 2),
                ),
            ];

            for (input, expected) in cases {
                assert_eq!(input.min_set(), expected, "Game {}", input.id);
            }
        }
    }

    static INPUT: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn p1() {
        let expected = 8;
        assert_eq!(part1(INPUT).unwrap(), format!("{expected}"));
    }

    #[test]
    fn p2() {
        let expected = 2286;
        assert_eq!(part2(INPUT).unwrap(), format!("{expected}"));
    }
}
