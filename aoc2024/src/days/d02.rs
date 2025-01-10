use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|level| level.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let num_safe = reports.iter().filter(|r| report_is_safe(r)).count();

    Ok(format!("{num_safe}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|level| level.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let num_safe = reports
        .iter()
        // For each report, if we can remove a level and get a report than indicates 'safe', then
        // treat the report as having indicated 'safe' to begin with
        .filter(|r| {
            if report_is_safe(r) {
                true
            } else {
                // Try removing each level sequentially to see if we get a safe report.
                // I can definitely be smarter about this, but the input is short enough this
                // should have a minimal impact on performance.
                let mut can_make_safe = false;

                for idx in 0..r.len() {
                    // Split the report at the given index, then combine the slices to a new
                    // report. Then, check the new report to determine if it's safe. If so, then we
                    // can treat the original report as safe.
                    let r = r[..idx]
                        .iter()
                        .map(|&l| l)
                        .chain(r[idx + 1..].iter().map(|&l| l))
                        .collect::<Vec<_>>();
                    if report_is_safe(&r) {
                        can_make_safe = true;
                        break; // No need to check the remaining reports.
                    }
                }

                can_make_safe
            }
        })
        .count();

    Ok(format!("{num_safe}"))
}

/// Determine if this report indicates safe or unsafe reactor levels.
/// Levels are safe iff:
/// * levels are only increasing _or_ decreasing
/// * differences between adjacent levels are at least one and at most three
fn report_is_safe(report: &[u32]) -> bool {
    let num_increasing = report.windows(2).filter(|w| w[0] < w[1]).count();
    let num_decreasing = report.windows(2).filter(|w| w[0] > w[1]).count();
    let bad_diffs = report
        .windows(2)
        .filter(|w| {
            let diff = w[0].max(w[1]) - w[0].min(w[1]);
            !(1 <= diff && diff <= 3)
        })
        .count();

    !(num_increasing > 0 && num_decreasing > 0) && bad_diffs == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "2");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "4");
    }

    #[test]
    fn report_safety() {
        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];

        for (input, expected) in cases {
            assert_eq!(report_is_safe(&input), expected, "Case: {input:?}");
        }
    }
}
