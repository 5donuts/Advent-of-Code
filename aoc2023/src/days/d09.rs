use std::error::Error;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let histories: Result<Vec<History>, _> = input.lines().map(|l| History::try_from(l)).collect();
    let sum: i32 = histories?.iter_mut().map(|h| h.next()).sum();
    Ok(format!("{sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let histories: Result<Vec<History>, _> = input.lines().map(|l| History::try_from(l)).collect();
    let sum: i32 = histories?.iter_mut().map(|h| h.prev()).sum();
    Ok(format!("{sum}"))
}

#[derive(Debug, PartialEq)]
struct History(Vec<Sequence>);

impl History {
    fn next(&mut self) -> i32 {
        if let Some(next) = self.0[0].next {
            next
        } else {
            for idx in (1..self.0.len()).rev() {
                let next_delta = self.0[idx].next.unwrap();
                self.0[idx - 1].next(next_delta);
            }
            self.0[0].next.unwrap()
        }
    }

    fn prev(&mut self) -> i32 {
        if let Some(prev) = self.0[0].prev {
            prev
        } else {
            for idx in (1..self.0.len()).rev() {
                let next_delta = self.0[idx].prev.unwrap();
                self.0[idx - 1].prev(next_delta);
            }
            self.0[0].prev.unwrap()
        }
    }
}

impl From<Sequence> for History {
    fn from(seq: Sequence) -> Self {
        let mut sequences = vec![seq];
        while sequences
            .last()
            .unwrap()
            .values
            .iter()
            .filter(|&&n| n != 0)
            .count()
            != 0
        {
            sequences.push(sequences.last().unwrap().diff());
        }

        Self(sequences)
    }
}

impl TryFrom<&str> for History {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let seq = Sequence::try_from(value)?;
        Ok(seq.into())
    }
}

#[derive(Debug, PartialEq)]
struct Sequence {
    values: Vec<i32>,
    next: Option<i32>,
    prev: Option<i32>,
}

impl Sequence {
    fn diff(&self) -> Self {
        let values: Vec<_> = self.values.windows(2).map(|w| w[1] - w[0]).collect();
        Self::from(values)
    }

    fn next(&mut self, delta: i32) -> i32 {
        if let Some(next) = self.next {
            next
        } else {
            let next = self.values.last().unwrap() + delta;
            self.next = Some(next);
            next
        }
    }

    fn prev(&mut self, delta: i32) -> i32 {
        if let Some(prev) = self.prev {
            prev
        } else {
            let prev = self.values.first().unwrap() - delta;
            self.prev = Some(prev);
            prev
        }
    }
}

impl From<Vec<i32>> for Sequence {
    fn from(value: Vec<i32>) -> Self {
        let (next, prev) = if value.iter().filter(|&&n| n != 0).count() > 0 {
            (None, None)
        } else {
            (Some(0), Some(0))
        };

        Self {
            values: value,
            next,
            prev,
        }
    }
}

impl TryFrom<&str> for Sequence {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let nums: Result<Vec<_>, _> = value.split_whitespace().map(|s| s.parse()).collect();
        Ok(Self::from(nums?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn p1() {
        let expected = 114;
        assert_eq!(
            part1(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    #[test]
    fn p2() {
        let expected = 2;
        assert_eq!(
            part2(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    mod history {
        use itertools::Itertools;

        use super::*;

        #[test]
        fn next() {
            let histories = INPUT
                .lines()
                .map(|l| History::try_from(l).unwrap())
                .collect_vec();
            let expected = vec![18, 28, 68];
            let cases = histories.into_iter().zip(expected);

            for (mut input, expected) in cases {
                assert_eq!(input.next(), expected, "actual != expected");
            }
        }

        #[test]
        fn prev() {
            let histories = INPUT
                .lines()
                .map(|l| History::try_from(l).unwrap())
                .collect_vec();
            let expected = vec![-3, 0, 5];
            let cases = histories.into_iter().zip(expected);

            for (mut input, expected) in cases {
                assert_eq!(input.prev(), expected, "actual != expected");
            }
        }

        #[test]
        fn from_seq() {
            let sequences = vec![
                Sequence::from(vec![0, 3, 6, 9, 12, 15]),
                Sequence::from(vec![1, 3, 6, 10, 15, 21]),
                Sequence::from(vec![10, 13, 16, 21, 30, 45]),
            ];
            let expected = vec![
                History(vec![
                    Sequence::from(vec![0, 3, 6, 9, 12, 15]),
                    Sequence::from(vec![3, 3, 3, 3, 3]),
                    Sequence::from(vec![0, 0, 0, 0]),
                ]),
                History(vec![
                    Sequence::from(vec![1, 3, 6, 10, 15, 21]),
                    Sequence::from(vec![2, 3, 4, 5, 6]),
                    Sequence::from(vec![1, 1, 1, 1]),
                    Sequence::from(vec![0, 0, 0]),
                ]),
                History(vec![
                    Sequence::from(vec![10, 13, 16, 21, 30, 45]),
                    Sequence::from(vec![3, 3, 5, 9, 15]),
                    Sequence::from(vec![0, 2, 4, 6]),
                    Sequence::from(vec![2, 2, 2]),
                    Sequence::from(vec![0, 0]),
                ]),
            ];
            let cases = sequences.into_iter().zip(expected);

            for (input, expected) in cases {
                assert_eq!(History::from(input), expected, "actual != expected");
            }
        }
    }

    mod sequence {
        use super::*;

        #[test]
        fn next() {
            let sequences = vec![
                (Sequence::from(vec![0, 3, 6, 9, 12, 15]), 3),
                (Sequence::from(vec![1, 3, 6, 10, 15, 21]), 7),
                (Sequence::from(vec![10, 13, 16, 21, 30, 45]), 23),
            ];
            let expected = vec![18, 28, 68];
            let cases = sequences.into_iter().zip(expected);

            for (input, expected) in cases {
                let (mut seq, delta) = input;
                assert_eq!(seq.next(delta), expected, "actual != expected");
            }
        }

        #[test]
        fn prev() {
            let sequences = vec![
                (Sequence::from(vec![0, 3, 6, 9, 12, 15]), 3),
                (Sequence::from(vec![1, 3, 6, 10, 15, 21]), 1),
                (Sequence::from(vec![10, 13, 16, 21, 30, 45]), 8),
            ];
            let expected = vec![-3, 0, 2];
            let cases = sequences.into_iter().zip(expected);

            for (input, expected) in cases {
                let (mut seq, delta) = input;
                assert_eq!(seq.prev(delta), expected, "actual != expected");
            }
        }

        #[test]
        fn parse() {
            let expected = vec![
                Sequence::from(vec![0, 3, 6, 9, 12, 15]),
                Sequence::from(vec![1, 3, 6, 10, 15, 21]),
                Sequence::from(vec![10, 13, 16, 21, 30, 45]),
            ];
            let cases = INPUT.lines().zip(expected);

            for (input, expected) in cases {
                assert_eq!(
                    Sequence::try_from(input).unwrap(),
                    expected,
                    "actual != expected"
                );
            }
        }

        #[test]
        fn diff() {
            let expected = vec![
                Sequence::from(vec![3, 3, 3, 3, 3]),
                Sequence::from(vec![2, 3, 4, 5, 6]),
                Sequence::from(vec![3, 3, 5, 9, 15]),
            ];
            let cases = INPUT
                .lines()
                .map(|l| Sequence::try_from(l).unwrap())
                .zip(expected);

            for (input, expected) in cases {
                assert_eq!(input.diff(), expected, "actual != expected");
            }
        }
    }
}
