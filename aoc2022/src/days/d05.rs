use std::error::Error;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let mut stacks = parse_stacks(input);
    let instructions = parse_instructions(input);

    for ins in instructions {
        for _ in 0..ins.cnt {
            let c = stacks[ins.src - 1].pop().unwrap();
            stacks[ins.dst - 1].push(c);
        }
    }

    let top_crates = stacks.iter().map(|s| s.crates().next().unwrap()).collect();
    Ok(top_crates)
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut stacks = parse_stacks(input);
    let instructions = parse_instructions(input);

    for ins in instructions {
        let mut tmp = Vec::new();
        for _ in 0..ins.cnt {
            let c = stacks[ins.src - 1].pop().unwrap();
            tmp.push(c);
        }

        for _ in 0..ins.cnt {
            let c = tmp.pop().unwrap();
            stacks[ins.dst - 1].push(c);
        }
    }

    let top_crates = stacks.iter().map(|s| s.crates().next().unwrap()).collect();
    Ok(top_crates)
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let input = input.replace("\r", "");
    let input = input.split("\n\n").next().unwrap();

    // grab the ID of the last stack from the last line in the stacks section of the input
    let num_stacks = input
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse::<u8>()
        .unwrap();

    // we build the stacks from the bottom-up so the crate order is correct
    let mut stacks: Vec<Stack> = (1..=num_stacks).map(|id| Stack::new(id)).collect();
    for line in input.lines().rev() {
        for (idx, c) in line.char_indices().filter(|(_, c)| c.is_ascii_uppercase()) {
            let stack_num = (idx as f32 / 4.).ceil() as usize;
            stacks
                .iter_mut()
                .filter(|s| s.id == stack_num as u8)
                .next()
                .unwrap()
                .push(c);
        }
    }

    stacks
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let input = input.replace("\r", "");
    let input = input.split("\n\n").skip(1).next().unwrap();

    input
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
struct Instruction {
    /// The number of crates to move
    cnt: usize,

    /// The ID of the stack with the crates to move
    src: usize,

    /// The ID of the stack to which to move the crates
    dst: usize,
}

// marginally less lazy than some other days
impl TryFrom<&str> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split(" ");

        parts.next().ok_or(String::from("source string invalid"))?; // skip 'move'
        let cnt = parts
            .next()
            .ok_or(String::from("source string invalid"))?
            .parse()?;

        parts.next().ok_or(String::from("source string invalid"))?; // skip 'from'
        let src = parts
            .next()
            .ok_or(String::from("source string invalid"))?
            .parse()?;

        parts.next().ok_or(String::from("source string invalid"))?; // skip 'to'
        let dst = parts
            .next()
            .ok_or(String::from("source string invalid"))?
            .parse()?;

        Ok(Self { cnt, src, dst })
    }
}

#[derive(Debug, PartialEq)]
struct Stack {
    id: u8,
    crates: Vec<char>,
}

impl Stack {
    fn new(id: u8) -> Self {
        Self {
            id,
            crates: Vec::new(),
        }
    }

    fn push(&mut self, c: char) {
        self.crates.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    /// Get all the crates, from the top of the stack to the bottom
    fn crates(&self) -> impl Iterator<Item = &char> + '_ {
        self.crates.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "    [D]    \n[N] [C]\n[Z] [M] [P]\n 1   2   3\n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "CMZ".to_string());
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "MCD".to_string());
    }

    #[test]
    fn build_stacks() {
        let expected = vec![
            Stack {
                id: 1,
                crates: vec!['Z', 'N'], // NB: the end of the vec is the top of the stack
            },
            Stack {
                id: 2,
                crates: vec!['M', 'C', 'D'],
            },
            Stack {
                id: 3,
                crates: vec!['P'],
            },
        ];

        let stacks = parse_stacks(INPUT);
        for (idx, stack) in stacks.iter().enumerate() {
            assert_eq!(stack, &expected[idx]);
        }
    }

    #[test]
    fn get_instructions() {
        let expected = vec![
            Instruction {
                cnt: 1,
                src: 2,
                dst: 1,
            },
            Instruction {
                cnt: 3,
                src: 1,
                dst: 3,
            },
            Instruction {
                cnt: 2,
                src: 2,
                dst: 1,
            },
            Instruction {
                cnt: 1,
                src: 1,
                dst: 2,
            },
        ];

        let instructions = parse_instructions(INPUT);
        for (idx, instruction) in instructions.iter().enumerate() {
            assert_eq!(instruction, &expected[idx]);
        }
    }
}
