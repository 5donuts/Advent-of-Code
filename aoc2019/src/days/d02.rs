use crate::intcode::Interpreter;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let mut program = input
        .split(",")
        .map(|d| d.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    program[1] = 12;
    program[2] = 2;

    let mut program = Interpreter::new(program);
    program.run_to_halt()?;

    let answer = program[0];
    Ok(format!("{answer}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let program = input
        .split(",")
        .map(|d| d.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let expected = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            let mut program = Interpreter::new(program);
            program.run_to_halt()?;

            if program[0] == expected {
                let answer = 100 * noun + verb;
                return Ok(format!("{answer}"));
            }
        }
    }

    Err("No matching noun/verb combinations found!".into())
}
