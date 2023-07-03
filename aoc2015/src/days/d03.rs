use std::collections::HashSet;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let directions: Vec<Direction> = input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut position = (0, 0);
    visited.insert(position); // need to add the origin

    for d in directions {
        let delta = d.delta();
        position = (position.0 + delta.0 as i32, position.1 + delta.1 as i32);
        visited.insert(position);
    }

    Ok(format!("{}", visited.len()))
}

pub fn part2(input: &str) -> PuzzleResult {
    let input = input.trim();
    let santa_directions: Vec<Direction> = input
        .chars()
        .step_by(2)
        .map(|c| c.try_into().unwrap())
        .collect();
    let robo_directions: Vec<Direction> = input
        .chars()
        .skip(1)
        .step_by(2)
        .map(|c| c.try_into().unwrap())
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_position = (0, 0);
    let mut robo_position = (0, 0);
    visited.insert(santa_position); // need to add the origin

    for d in santa_directions {
        let delta = d.delta();
        santa_position = (
            santa_position.0 + delta.0 as i32,
            santa_position.1 + delta.1 as i32,
        );
        visited.insert(santa_position);
    }

    for d in robo_directions {
        let delta = d.delta();
        robo_position = (
            robo_position.0 + delta.0 as i32,
            robo_position.1 + delta.1 as i32,
        );
        visited.insert(robo_position);
    }

    Ok(format!("{}", visited.len()))
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn delta(&self) -> (i8, i8) {
        use Direction::*;
        match self {
            North => (0, 1),
            South => (0, -1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match c {
            '^' => Ok(North),
            'v' => Ok(South),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(format!("Unknown: '{}'", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];
        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{}", expected));
        }
    }
}
