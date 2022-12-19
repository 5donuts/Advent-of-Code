use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let total_score: usize = input
        .lines()
        .into_iter()
        .map(|l| {
            let mut moves = l.chars();
            let opp = moves.next().unwrap();
            moves.next().unwrap(); // skip the space
            let you = moves.next().unwrap();
            let round = Round::new(Shape::from(you), Shape::from(opp));

            round.value()
        })
        .sum();

    Ok(format!("{}", total_score))
}

pub fn part2(input: &str) -> PuzzleResult {
    let total_score: usize = input
        .lines()
        .into_iter()
        .map(|l| {
            let mut input = l.chars();
            let opp = input.next().unwrap();
            input.next().unwrap(); // skip the space
            let out = input.next().unwrap();

            let opp = Shape::from(opp);
            let out = Outcome::from(out);
            let you = Shape::for_outcome(&opp, &out);
            let round = Round { you, out }; // no need to calculate the outcome

            round.value()
        })
        .sum();

    Ok(format!("{}", total_score))
}

struct Round {
    you: Shape,
    out: Outcome,
}

impl Round {
    fn new(you: Shape, opp: Shape) -> Self {
        let out = Self::outcome(&opp, &you);
        Self { you, out }
    }

    fn value(&self) -> usize {
        self.you.value() + self.out.value()
    }

    fn outcome(opp: &Shape, you: &Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;

        if opp == you {
            Draw
        } else {
            match you {
                Rock => match opp {
                    Scissors => Win,
                    Paper => Lose,
                    _ => unimplemented!(),
                },
                Paper => match opp {
                    Rock => Win,
                    Scissors => Lose,
                    _ => unimplemented!(),
                },
                Scissors => match opp {
                    Rock => Lose,
                    Paper => Win,
                    _ => unimplemented!(),
                },
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> usize {
        use Shape::*;
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn for_outcome(opp: &Shape, out: &Outcome) -> Self {
        use Outcome::*;
        use Shape::*;

        if out == &Draw {
            *opp
        } else {
            match opp {
                Rock => match out {
                    Win => Paper,
                    Lose => Scissors,
                    _ => unimplemented!(),
                },
                Paper => match out {
                    Win => Scissors,
                    Lose => Rock,
                    _ => unimplemented!(),
                },
                Scissors => match out {
                    Win => Rock,
                    Lose => Paper,
                    _ => unimplemented!(),
                },
            }
        }
    }
}

// I'm too lazy for TryFrom
impl From<char> for Shape {
    fn from(c: char) -> Self {
        use Shape::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            c => panic!("Got invalid char: '{}'", c),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> usize {
        use Outcome::*;
        match *self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

// I'm too lazy for TryFrom
impl From<char> for Outcome {
    fn from(c: char) -> Self {
        use Outcome::*;
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            c => panic!("Got invalid char: '{}'", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"A Y
B X
C Z"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "15".to_string());
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "12".to_string());
    }

    #[test]
    fn round_outcomes() {
        use Outcome::*;
        use Shape::*;

        assert_eq!(Round::outcome(&Paper, &Rock), Lose);
        assert_eq!(Round::outcome(&Scissors, &Rock), Win);
        assert_eq!(Round::outcome(&Rock, &Rock), Draw);
        assert_eq!(Round::outcome(&Rock, &Paper), Win);
        assert_eq!(Round::outcome(&Rock, &Scissors), Lose);
    }

    #[test]
    fn find_shape_for_outcome() {
        use Outcome::*;
        use Shape::*;

        assert_eq!(Shape::for_outcome(&Paper, &Lose), Rock);
        assert_eq!(Shape::for_outcome(&Scissors, &Win), Rock);
        assert_eq!(Shape::for_outcome(&Rock, &Draw), Rock);
        assert_eq!(Shape::for_outcome(&Rock, &Win), Paper);
        assert_eq!(Shape::for_outcome(&Rock, &Lose), Scissors);
    }
}
