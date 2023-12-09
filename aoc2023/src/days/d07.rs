use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    use p1::Hand;

    let hands: Result<Vec<Hand>, _> = input.lines().map(|l| Hand::try_from(l)).collect();
    let total_winnings = total_winnings(hands?);
    Ok(format!("{total_winnings}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    use p2::Hand;

    let hands: Result<Vec<Hand>, _> = input.lines().map(|l| Hand::try_from(l)).collect();
    let total_winnings = total_winnings(hands?);
    Ok(format!("{total_winnings}"))
}

fn total_winnings(hands: Vec<impl CamelCardsHand>) -> u32 {
    let hands = {
        let mut hands = hands;
        hands.sort_unstable();
        hands
    };

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid() * (idx + 1) as u32)
        .sum()
}

trait CamelCardsHand: Clone + Ord {
    fn bid(&self) -> u32;
    fn hand_type(&self) -> HandType;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card(char);

impl TryFrom<char> for Card {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card('A')),
            'K' => Ok(Card('K')),
            'Q' => Ok(Card('Q')),
            'J' => Ok(Card('J')),
            'T' => Ok(Card('T')),
            '9' => Ok(Card('9')),
            '8' => Ok(Card('8')),
            '7' => Ok(Card('7')),
            '6' => Ok(Card('6')),
            '5' => Ok(Card('5')),
            '4' => Ok(Card('4')),
            '3' => Ok(Card('3')),
            '2' => Ok(Card('2')),
            _ => Err(format!("Invalid card: {c}").into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        use HandType::*;
        use Ordering::*;

        match self {
            FiveKind => match other {
                FiveKind => Equal,
                _ => Greater,
            },
            FourKind => match other {
                FiveKind => Less,
                FourKind => Equal,
                _ => Greater,
            },
            FullHouse => match other {
                FiveKind | FourKind => Less,
                FullHouse => Equal,
                _ => Greater,
            },
            ThreeKind => match other {
                FiveKind | FourKind | FullHouse => Less,
                ThreeKind => Equal,
                _ => Greater,
            },
            TwoPair => match other {
                OnePair | HighCard => Greater,
                TwoPair => Equal,
                _ => Less,
            },
            OnePair => match other {
                HighCard => Greater,
                OnePair => Equal,
                _ => Less,
            },
            HighCard => match other {
                HighCard => Equal,
                _ => Less,
            },
        }
    }
}

/// Parse a single line of input into the components of a [`CamelCardsHand`]
fn parse_line(l: &str) -> Result<(Vec<Card>, u32), Box<dyn Error>> {
    let cards: Result<Vec<Card>, _> = l
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("No cards component for hand: {l}"))?
        .chars()
        .map(|c| Card::try_from(c))
        .collect();

    let bid: Result<u32, _> = l
        .split_whitespace()
        .skip(1)
        .next()
        .ok_or_else(|| format!("No bid component for hand: {l}"))?
        .parse();

    Ok((cards?, bid?))
}

mod p1 {
    use super::*;

    fn card_cmp(this: &Card, other: &Card) -> Ordering {
        use Ordering::*;

        let this = this.0;
        let other = other.0;

        if this.is_numeric() && other.is_numeric() {
            this.cmp(&other)
        } else if this.is_alphabetic() && other.is_numeric() {
            Greater
        } else if this.is_numeric() && other.is_alphabetic() {
            Less
        } else {
            match this {
                'A' => match other {
                    'A' => Equal,
                    _ => Greater,
                },
                'K' => match other {
                    'A' => Less,
                    'K' => Equal,
                    _ => Greater,
                },
                'Q' => match other {
                    'A' | 'K' => Less,
                    'Q' => Equal,
                    _ => Greater,
                },
                'J' => match other {
                    'T' => Greater,
                    'J' => Equal,
                    _ => Less,
                },
                'T' => match other {
                    'T' => Equal,
                    _ => Less,
                },
                _ => panic!("Invalid branch!"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Hand {
        cards: Vec<Card>,
        bid: u32,
    }

    impl CamelCardsHand for Hand {
        fn bid(&self) -> u32 {
            self.bid
        }

        fn hand_type(&self) -> HandType {
            let mut counts = HashMap::new();

            for card in &self.cards {
                let label = card.0;
                let count = if let Some(count) = counts.get(&label) {
                    count + 1
                } else {
                    1
                };
                counts.insert(label, count);
            }

            let unique_labels = counts.keys().count();
            match unique_labels {
                1 => HandType::FiveKind,
                2 => {
                    let (c1, c2) = counts.values().collect_tuple().unwrap();

                    if *c1 == 4 || *c2 == 4 {
                        HandType::FourKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    let (c1, c2, c3) = counts.values().collect_tuple().unwrap();

                    if *c1 == 3 || *c2 == 3 || *c3 == 3 {
                        HandType::ThreeKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!("Invalid Hand!"),
            }
        }
    }

    impl TryFrom<&str> for Hand {
        type Error = Box<dyn Error>;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let (cards, bid) = parse_line(value)?;
            Ok(Self { cards, bid })
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            use Ordering::*;

            let primary_ordering = self.hand_type().cmp(&other.hand_type());

            if primary_ordering != Equal {
                primary_ordering
            } else if let Some(secondary_ordering) = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s_card, o_card)| card_cmp(s_card, o_card))
                .filter(|ordering| ordering != &Equal)
                .next()
            {
                secondary_ordering
            } else {
                Equal
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rand::seq::SliceRandom;

        static INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        #[test]
        fn card_ord() {
            let mut rng = rand::thread_rng();

            let expected = vec![
                Card('A'),
                Card('K'),
                Card('Q'),
                Card('J'),
                Card('T'),
                Card('9'),
                Card('8'),
                Card('7'),
                Card('6'),
                Card('5'),
                Card('4'),
                Card('3'),
                Card('2'),
            ];

            let mut actual = expected.clone();
            actual.shuffle(&mut rng);
            actual.sort_unstable_by(card_cmp);
            actual.reverse(); // for descending order

            assert_eq!(actual, expected, "actual != expected");
        }

        mod hand {
            use super::*;

            #[test]
            fn parse() {
                let expected = vec![
                    Hand {
                        cards: vec![Card('3'), Card('2'), Card('T'), Card('3'), Card('K')],
                        bid: 765,
                    },
                    Hand {
                        cards: vec![Card('T'), Card('5'), Card('5'), Card('J'), Card('5')],
                        bid: 684,
                    },
                    Hand {
                        cards: vec![Card('K'), Card('K'), Card('6'), Card('7'), Card('7')],
                        bid: 28,
                    },
                    Hand {
                        cards: vec![Card('K'), Card('T'), Card('J'), Card('J'), Card('T')],
                        bid: 220,
                    },
                    Hand {
                        cards: vec![Card('Q'), Card('Q'), Card('Q'), Card('J'), Card('A')],
                        bid: 483,
                    },
                ];
                let cases = INPUT.lines().zip(expected.iter());

                for (input, expected) in cases {
                    assert_eq!(
                        Hand::try_from(input).unwrap(),
                        *expected,
                        "actual != expected"
                    );
                }
            }

            #[test]
            fn hand_type() {
                use HandType::*;
                let cases = vec![
                    ("AAAAA 0", FiveKind),
                    ("AA8AA 0", FourKind),
                    ("23332 0", FullHouse),
                    ("TTT98 0", ThreeKind),
                    ("23432 0", TwoPair),
                    ("A23A4 0", OnePair),
                    ("23456 0", HighCard),
                ];

                for (input, expected) in cases {
                    assert_eq!(
                        Hand::try_from(input).unwrap().hand_type(),
                        expected,
                        "actual != expected for: {input}"
                    );
                }
            }

            #[test]
            fn ord() {
                let mut rng = rand::thread_rng();

                // primary ordering
                let expected = vec![
                    Hand::try_from("AAAAA 0").unwrap(),
                    Hand::try_from("AA8AA 0").unwrap(),
                    Hand::try_from("23332 0").unwrap(),
                    Hand::try_from("TTT98 0").unwrap(),
                    Hand::try_from("23432 0").unwrap(),
                    Hand::try_from("A23A4 0").unwrap(),
                    Hand::try_from("23456 0").unwrap(),
                ];

                let mut actual = expected.clone();
                actual.shuffle(&mut rng);
                actual.sort_unstable();
                actual.reverse(); // for descending order

                assert_eq!(actual, expected, "Primary ordering: actual != expected");

                // secondary ordering
                let expected = vec![
                    Hand::try_from("33332 0").unwrap(),
                    Hand::try_from("2AAAA 0").unwrap(),
                    Hand::try_from("77888 0").unwrap(),
                    Hand::try_from("77788 0").unwrap(),
                ];

                let mut actual = expected.clone();
                actual.shuffle(&mut rng);
                actual.sort_unstable();
                actual.reverse(); // for descending order

                assert_eq!(actual, expected, "Secondary ordering: actual != expected");
            }
        }
    }
}

mod p2 {
    use super::*;

    fn card_cmp(this: &Card, other: &Card) -> Ordering {
        use Ordering::*;

        let this = this.0;
        let other = other.0;

        if this == 'J' && other == 'J' {
            Equal
        } else if this == 'J' && other != 'J' {
            Less
        } else if this != 'J' && other == 'J' {
            Greater
        } else if this.is_numeric() && other.is_numeric() {
            this.cmp(&other)
        } else if this.is_alphabetic() && other.is_numeric() {
            Greater
        } else if this.is_numeric() && other.is_alphabetic() {
            Less
        } else {
            match this {
                'A' => match other {
                    'A' => Equal,
                    _ => Greater,
                },
                'K' => match other {
                    'A' => Less,
                    'K' => Equal,
                    _ => Greater,
                },
                'Q' => match other {
                    'A' | 'K' => Less,
                    'Q' => Equal,
                    _ => Greater,
                },
                'T' => match other {
                    'T' => Equal,
                    _ => Less,
                },
                _ => panic!("Invalid branch!"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Hand {
        cards: Vec<Card>,
        bid: u32,
    }

    impl CamelCardsHand for Hand {
        fn bid(&self) -> u32 {
            self.bid
        }

        fn hand_type(&self) -> HandType {
            let mut counts = HashMap::new();

            for card in &self.cards {
                let label = card.0;
                let count = if let Some(count) = counts.get(&label) {
                    count + 1
                } else {
                    1
                };
                counts.insert(label, count);
            }

            let unique_labels = counts.keys().count();
            let joker = counts.keys().filter(|&&k| k == 'J').count() > 0;
            match unique_labels {
                1 => HandType::FiveKind,
                2 => {
                    if joker {
                        HandType::FiveKind
                    } else {
                        let (c1, c2) = counts.values().collect_tuple().unwrap();

                        if *c1 == 4 || *c2 == 4 {
                            HandType::FourKind
                        } else {
                            HandType::FullHouse
                        }
                    }
                }
                3 => {
                    if joker {
                        let (c1, c2) = counts
                            .iter()
                            .filter_map(|(&k, v)| if k != 'J' { Some(v) } else { None })
                            .collect_tuple()
                            .unwrap();
                        if *c1 == 1 || *c2 == 1 {
                            HandType::FourKind
                        } else {
                            HandType::FullHouse
                        }
                    } else {
                        let (c1, c2, c3) = counts.values().collect_tuple().unwrap();

                        if *c1 == 3 || *c2 == 3 || *c3 == 3 {
                            HandType::ThreeKind
                        } else {
                            HandType::TwoPair
                        }
                    }
                }
                4 => {
                    if joker {
                        let (c1, c2, c3) = counts
                            .iter()
                            .filter_map(|(&k, v)| if k != 'J' { Some(v) } else { None })
                            .collect_tuple()
                            .unwrap();
                        if *c1 == 1 || *c2 == 1 || *c3 == 1 {
                            HandType::ThreeKind
                        } else {
                            HandType::TwoPair
                        }
                    } else {
                        HandType::OnePair
                    }
                }
                5 => {
                    if joker {
                        HandType::OnePair
                    } else {
                        HandType::HighCard
                    }
                }
                _ => panic!("Invalid Hand!"),
            }
        }
    }

    impl TryFrom<&str> for Hand {
        type Error = Box<dyn Error>;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let (cards, bid) = parse_line(value)?;
            Ok(Self { cards, bid })
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            use Ordering::*;

            let primary_ordering = self.hand_type().cmp(&other.hand_type());

            if primary_ordering != Equal {
                primary_ordering
            } else if let Some(secondary_ordering) = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s_card, o_card)| card_cmp(s_card, o_card))
                .filter(|ordering| ordering != &Equal)
                .next()
            {
                secondary_ordering
            } else {
                Equal
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rand::seq::SliceRandom;

        static INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        #[test]
        fn card_ord() {
            let mut rng = rand::thread_rng();

            let expected = vec![
                Card('A'),
                Card('K'),
                Card('Q'),
                Card('T'),
                Card('9'),
                Card('8'),
                Card('7'),
                Card('6'),
                Card('5'),
                Card('4'),
                Card('3'),
                Card('2'),
                Card('J'),
            ];

            let mut actual = expected.clone();
            actual.shuffle(&mut rng);
            actual.sort_unstable_by(card_cmp);
            actual.reverse(); // for descending order

            assert_eq!(actual, expected, "actual != expected");
        }

        mod hand {
            use super::*;

            #[test]
            fn parse() {
                let expected = vec![
                    Hand {
                        cards: vec![Card('3'), Card('2'), Card('T'), Card('3'), Card('K')],
                        bid: 765,
                    },
                    Hand {
                        cards: vec![Card('T'), Card('5'), Card('5'), Card('J'), Card('5')],
                        bid: 684,
                    },
                    Hand {
                        cards: vec![Card('K'), Card('K'), Card('6'), Card('7'), Card('7')],
                        bid: 28,
                    },
                    Hand {
                        cards: vec![Card('K'), Card('T'), Card('J'), Card('J'), Card('T')],
                        bid: 220,
                    },
                    Hand {
                        cards: vec![Card('Q'), Card('Q'), Card('Q'), Card('J'), Card('A')],
                        bid: 483,
                    },
                ];
                let cases = INPUT.lines().zip(expected.iter());

                for (input, expected) in cases {
                    assert_eq!(
                        Hand::try_from(input).unwrap(),
                        *expected,
                        "actual != expected"
                    );
                }
            }

            #[test]
            fn hand_type() {
                use HandType::*;

                let cases = vec![
                    ("KKKKK 0", FiveKind),
                    ("KKKKJ 0", FiveKind),
                    ("KKKJJ 0", FiveKind),
                    ("KKJJJ 0", FiveKind),
                    ("KJJJJ 0", FiveKind),
                    ("JJJJJ 0", FiveKind),
                    ("KQQQQ 0", FourKind),
                    ("KQQQJ 0", FourKind),
                    ("KQQJJ 0", FourKind),
                    ("KQJJJ 0", FourKind),
                    ("KJJJJ 0", FiveKind),
                    ("KKQQQ 0", FullHouse),
                    ("KKQQJ 0", FullHouse),
                    ("KKQJJ 0", FourKind),
                    ("KQQQT 0", ThreeKind),
                    ("KQQJT 0", ThreeKind),
                    ("KQJJT 0", ThreeKind),
                    ("KJJJT 0", FourKind),
                    ("KKQQT 0", TwoPair),
                    ("KQJT9 0", OnePair),
                    ("KKQJT 0", ThreeKind),
                    ("KQT98 0", HighCard),
                ];

                for (input, expected) in cases {
                    assert_eq!(
                        Hand::try_from(input).unwrap().hand_type(),
                        expected,
                        "actual != expected for: {input}"
                    );
                }
            }

            #[test]
            fn ord() {
                let mut rng = rand::thread_rng();

                let expected = vec![
                    Hand::try_from("KTJJT 220").unwrap(),
                    Hand::try_from("QQQJA 483").unwrap(),
                    Hand::try_from("T55J5 684").unwrap(),
                    Hand::try_from("KK677 28").unwrap(),
                    Hand::try_from("32T3K 765").unwrap(),
                ];

                let mut actual = expected.clone();
                actual.shuffle(&mut rng);
                actual.sort_unstable();
                actual.reverse(); // for descending order

                assert_eq!(actual, expected, "actual != expected");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::seq::SliceRandom;

    static INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn p1() {
        let expected = 6440;
        assert_eq!(
            part1(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    #[test]
    fn p2() {
        let expected = 5905;
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
            let valid = vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ];
            let invalid = vec!['1', 'X', 'Z'];

            for c in valid {
                assert!(Card::try_from(c).is_ok());
            }

            for c in invalid {
                assert!(Card::try_from(c).is_err());
            }
        }
    }

    mod hand_type {
        use super::*;

        #[test]
        fn ord() {
            use HandType::*;
            let mut rng = rand::thread_rng();

            let expected = vec![
                FiveKind, FourKind, FullHouse, ThreeKind, TwoPair, OnePair, HighCard,
            ];

            let mut actual = expected.clone();
            actual.shuffle(&mut rng);
            actual.sort_unstable();
            actual.reverse(); // for descending order

            assert_eq!(actual, expected, "actual != expected");
        }
    }
}
