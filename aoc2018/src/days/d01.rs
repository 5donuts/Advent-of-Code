use utils::PuzzleResult;

use std::collections::HashSet;

pub fn part1(input: &str) -> PuzzleResult {
    let deltas: Vec<i32> = input
        .lines()
        .flat_map(|l| {
            l.split(", ").map(|c| {
                let c = if c.starts_with("+") { &c[1..] } else { c };
                c.parse::<i32>()
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let frequency: i32 = deltas.iter().sum();

    Ok(format!("{}", frequency))
}

pub fn part2(input: &str) -> PuzzleResult {
    let deltas: Vec<i32> = input
        .lines()
        .flat_map(|l| {
            l.split(", ").map(|c| {
                let c = if c.starts_with("+") { &c[1..] } else { c };
                c.parse::<i32>()
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Because we may have to repeat the deltas many times before a repeat frequency is found,
    // store the visited frequencies in a HashSet to provide O(1) lookups.
    let mut last_freq = 0;
    let mut frequencies: HashSet<i32> = HashSet::default();
    frequencies.insert(last_freq);

    // Loop through the list of deltas until a repeat frequency is found
    let mut first_repeat = None;
    let mut idx = 0;
    while first_repeat.is_none() {
        let prev = last_freq;
        let next = prev
            + deltas
                .get(idx % deltas.len())
                .ok_or_else(|| format!("No delta at idx {} ({idx})", idx % deltas.len()))?;
        last_freq = next;
        idx += 1;

        if let Some(_) = frequencies.get(&next) {
            first_repeat = Some(next);
            break;
        } else {
            frequencies.insert(next);
        }
    }

    Ok(format!("{}", first_repeat.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![
            ("+1, -2, +3, +1", 3),
            ("+1, +1, +1", 3),
            ("+1, +1, -2", 0),
            ("-1, -2, -3", -6),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![
            ("+1, -2, +3, +1, +1, -2", 2),
            ("+1, -1", 0),
            ("+3, +3, +4, -2, -4", 10),
            ("-6, +3, +8, +5, -6", 5),
            ("+7, +7, -2, -7, -4", 14),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{}", expected));
        }
    }
}
