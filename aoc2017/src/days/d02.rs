use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let rows = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let diff_sum = rows
        .iter()
        .zip(rows.iter())
        .map(|(l, r)| l.iter().max().unwrap() - r.iter().min().unwrap())
        .sum::<u32>();

    Ok(format!("{diff_sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut rows = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let rows = rows
        .iter_mut()
        .map(|row| {
            let row = row;
            // Sorting the rows lets us skip a lot of calculations when checking divisibility
            // because we know that for two numbers x, y such that y > x, x % y != 0.
            row.sort_unstable();
            row
        })
        .collect::<Vec<_>>();

    let quotient_sum: u32 = rows
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(idx, d)| {
                    // If there are no remaining numbers to check, this number has no multiples
                    if idx == row.len() {
                        None
                    } else {
                        // Check all numbers that come after the current number for potential
                        // multiples
                        row[idx + 1..]
                            .iter()
                            .filter(|&x| x % d == 0)
                            .next()
                            .and_then(|x| Some(x / d))
                    }
                })
                .next()
                .unwrap()
        })
        .sum();

    Ok(format!("{quotient_sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = r#"5 1 9 5
7 5 3
2 4 6 8"#;
        assert_eq!(part1(input).unwrap(), "18");
    }

    #[test]
    fn p2() {
        let input = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;
        assert_eq!(part2(input).unwrap(), "9");
    }
}
