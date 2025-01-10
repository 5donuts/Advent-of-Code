use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let instructions = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let dir = it.next().ok_or_else(|| format!("No dir in line: '{l}'"))?;
            let dist = it
                .next()
                .ok_or_else(|| format!("No dist in line: '{l}'"))?
                .parse::<i32>()?;

            Ok((dir, dist))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    let mut h_dist = 0;
    let mut depth = 0;
    for (instruction, distance) in instructions {
        match instruction {
            "forward" => h_dist += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => return Err(format!("Unexpected instruction: {instruction}").into()),
        }
    }

    let answer = h_dist * depth;
    Ok(format!("{answer}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let instructions = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let dir = it.next().ok_or_else(|| format!("No dir in line: '{l}'"))?;
            let dist = it
                .next()
                .ok_or_else(|| format!("No dist in line: '{l}'"))?
                .parse::<i32>()?;

            Ok((dir, dist))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    let mut h_dist = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (instruction, distance) in instructions {
        match instruction {
            "forward" => {
                h_dist += distance;
                depth += aim * distance;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => return Err(format!("Unexpected instruction: {instruction}").into()),
        }
    }

    let answer = h_dist * depth;
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "150");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "900");
    }
}
