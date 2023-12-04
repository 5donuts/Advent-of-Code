use std::collections::VecDeque;
use std::error::Error;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let cards: Result<Vec<Card>, _> = input.lines().map(|l| Card::try_from(l)).collect();
    let total_points: u32 = cards?.iter().map(|c| c.points()).sum();

    Ok(format!("{total_points}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let cards: Result<Vec<Card>, _> = input.lines().map(|l| Card::try_from(l)).collect();
    let cards = cards?;

    // While processing the cards in our queue, we use this map of Card -> Copies
    // to speed up processing. The map is indexed by Card ID.
    let mut copy_map = vec![None; cards.len() + 1];

    // We also keep a running total of the number of scratch cards, indexed by
    // Card ID.
    let mut card_count = vec![0; cards.len() + 1];

    let mut queue = VecDeque::new();
    queue.extend(cards.iter());
    while let Some(card) = queue.pop_front() {
        card_count[card.id()] += 1;

        let copies = if let Some(copies) = &copy_map[card.id()] {
            copies
        } else {
            copy_map[card.id()] = Some(card.copies(&cards));
            copy_map[card.id()].as_ref().unwrap()
        };
        queue.extend(copies.iter());
    }

    let total_cards: u32 = card_count.iter().sum();
    Ok(format!("{total_cards}"))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    id: usize,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn id(&self) -> usize {
        self.id
    }

    fn points(&self) -> u32 {
        let matches = self.matches().len();

        if matches == 0 {
            0
        } else if matches == 1 {
            1
        } else {
            1 << (matches - 1)
        }
    }

    fn matches(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter_map(|n| {
                if self.winners.contains(n) {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect()
    }

    /// In the given list of cards, return the list of card copies won
    /// by this card. If there are no matches, return an empty [`Vec`].
    fn copies<'a>(&self, cards: &'a [Card]) -> Vec<&'a Card> {
        let matches = self.matches().len();

        if matches == 0 {
            Vec::new()
        } else {
            // Note: `cards` is 0-indexed, but Card IDs are 1-indexed
            (self.id()..(self.id() + matches))
                .into_iter()
                .map(|id| &cards[id])
                .collect()
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id: usize = value
            .split(":")
            .next()
            .ok_or_else(|| format!("No 'Card XX:' part in {value}"))?
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .skip(1)
            .next()
            .ok_or_else(|| format!("No number following 'Card ' in {value}"))?
            .parse()?;

        let winning_numbers: Result<Vec<u32>, _> = value
            .split(":")
            .skip(1)
            .next()
            .ok_or_else(|| format!("No numbers part in {value}"))?
            .split("|")
            .next()
            .ok_or_else(|| format!("No '|' separator in {value}"))?
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .map(|s| s.trim().parse())
            .collect();

        let numbers: Result<Vec<u32>, _> = value
            .split(":")
            .skip(1)
            .next()
            .ok_or_else(|| format!("No numbers part in {value}"))?
            .split("|")
            .skip(1)
            .next()
            .ok_or_else(|| format!("No post-'|' part in {value}"))?
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .map(|s| s.parse())
            .collect();

        Ok(Self {
            id,
            numbers: numbers?,
            winners: winning_numbers?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn p1() {
        let expected = 13;
        assert_eq!(
            part1(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    #[test]
    fn p2() {
        let expected = 30;
        assert_eq!(
            part2(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    mod card {
        use super::*;

        #[test]
        fn parse() {
            let expected = vec![
                Card {
                    id: 1,
                    winners: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
                Card {
                    id: 2,
                    winners: vec![13, 32, 20, 16, 61],
                    numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
                Card {
                    id: 3,
                    winners: vec![1, 21, 53, 59, 44],
                    numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                },
                Card {
                    id: 4,
                    winners: vec![41, 92, 73, 84, 69],
                    numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                },
                Card {
                    id: 5,
                    winners: vec![87, 83, 26, 28, 32],
                    numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                },
                Card {
                    id: 6,
                    winners: vec![31, 18, 13, 56, 72],
                    numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                },
            ];
            let cases = INPUT.lines().zip(expected.iter());

            for (input, expected) in cases {
                let card = Card::try_from(input);
                assert_eq!(
                    &card.unwrap(),
                    expected,
                    "actual != expected for card {}",
                    expected.id()
                );
            }
        }

        #[test]
        fn matches() {
            let expected_matches = vec![
                vec![48, 83, 86, 17],
                vec![32, 61],
                vec![1, 21],
                vec![84],
                vec![],
                vec![],
            ];
            let cases = INPUT.lines().zip(expected_matches.iter());

            for (input, expected) in cases {
                let card = Card::try_from(input).unwrap();

                let actual = {
                    let mut matches = card.matches();
                    matches.sort();
                    matches
                };

                let expected = {
                    let mut matches = expected.to_owned();
                    matches.sort();
                    matches
                };

                assert_eq!(
                    actual,
                    expected,
                    "actual != expected for card {}",
                    card.id()
                );
            }
        }

        #[test]
        fn points() {
            let expected_points = vec![8, 2, 2, 1, 0, 0];
            let cases = INPUT.lines().zip(expected_points.iter());

            for (input, &expected) in cases {
                let card = Card::try_from(input).unwrap();
                assert_eq!(
                    card.points(),
                    expected,
                    "actual != expected for card {}",
                    card.id()
                );
            }
        }

        #[test]
        fn copies() {
            let cards: Result<Vec<Card>, _> = INPUT.lines().map(|l| Card::try_from(l)).collect();
            let cards = cards.unwrap();

            let expected_copies = vec![
                vec![&cards[1], &cards[2], &cards[3], &cards[4]], // remember 0-indexing!
                vec![&cards[2], &cards[3]],
                vec![&cards[3], &cards[4]],
                vec![&cards[4]],
                Vec::new(),
                Vec::new(),
            ];
            let cases = INPUT.lines().zip(expected_copies.iter());

            for (input, expected) in cases {
                let card = Card::try_from(input).unwrap();

                let expected = {
                    let mut copies = expected.to_owned();
                    copies.sort();
                    copies
                };

                let actual = {
                    let mut copies = card.copies(&cards);
                    copies.sort();
                    copies
                };

                assert_eq!(actual, expected, "actual != expected for card {}", card.id);
            }
        }
    }
}
