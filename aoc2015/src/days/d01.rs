use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let instructions: Vec<Instruction> = input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();
    let mut floor = 0;
    for i in instructions {
        use Instruction::*;
        match i {
            Up => floor += 1,
            Down => floor -= 1,
        }
    }

    Ok(format!("{}", floor))
}

pub fn part2(input: &str) -> PuzzleResult {
    let instructions: Vec<Instruction> = input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();
    let mut floor = 0;
    for (idx, i) in instructions.iter().enumerate() {
        use Instruction::*;
        match i {
            Up => floor += 1,
            Down => floor -= 1,
        }

        if floor == -1 {
            return Ok(format!("{}", idx + 1));
        }
    }

    Err(format!("Never found the basement!"))
}

enum Instruction {
    Up,
    Down,
}

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Instruction::*;

        match c {
            '(' => Ok(Up),
            ')' => Ok(Down),
            _ => Err(format!("Unknown: '{}'", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![(")", 1), ("()())", 5)];
        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{}", expected));
        }
    }
}
