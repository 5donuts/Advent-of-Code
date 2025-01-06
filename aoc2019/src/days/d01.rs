use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let masses = input
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let fuel_sum = masses.iter().map(|m| m / 3 - 2).sum::<u32>();

    Ok(format!("{fuel_sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let masses = input
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let fuel_sum = masses.iter().map(|&m| get_fuel(m)).flatten().sum::<i32>();

    Ok(format!("{fuel_sum}"))
}

/// Get an [Iterator] that yields the fuel masses required to lift the given mass
fn get_fuel(mass: i32) -> impl std::iter::Iterator<Item = i32> {
    let mut remaining_mass = mass;

    std::iter::from_fn(move || {
        remaining_mass = remaining_mass / 3 - 2;
        if remaining_mass > 0 {
            Some(remaining_mass)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![("12", 2), ("14", 2), ("1969", 654), ("100756", 33583)];

        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{expected}"));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![("14", 2), ("1969", 966), ("100756", 50346)];

        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{expected}"));
        }
    }
}
