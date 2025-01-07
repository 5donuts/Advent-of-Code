use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let measurements = input
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let increase_count = measurements
        .windows(2)
        .filter_map(|w| {
            let prev = w[0];
            let next = w[1];

            if next > prev {
                Some(())
            } else {
                None
            }
        })
        .count();

    Ok(format!("{increase_count}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let measurements = input
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let sliding_window_sums: Vec<_> = measurements
        .iter()
        .enumerate()
        .filter_map(|(idx, m1)| {
            let m2 = measurements.get(idx + 1);
            let m3 = measurements.get(idx + 2);

            if m2.is_none() || m3.is_none() {
                None
            } else {
                Some(m1 + m2.unwrap() + m3.unwrap())
            }
        })
        .collect();

    let increase_count = sliding_window_sums
        .windows(2)
        .filter_map(|w| {
            let prev = w[0];
            let next = w[1];

            if next > prev {
                Some(())
            } else {
                None
            }
        })
        .count();

    Ok(format!("{increase_count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn p1() {
        let expected = 7;
        assert_eq!(part1(INPUT).unwrap(), format!("{expected}"));
    }

    #[test]
    fn p2() {
        let expected = 5;
        assert_eq!(part2(INPUT).unwrap(), format!("{expected}"));
    }
}
