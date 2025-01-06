use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let digits = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| format!("Unable to parse char to digit: {c}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let first = digits.first().ok_or("No first digit")?;
    let last = digits.last().ok_or("No last digit")?;
    let sum = if first == last { *first } else { 0 }
        + digits
            .windows(2)
            .filter_map(|digits| {
                if digits[0] == digits[1] {
                    Some(digits[0])
                } else {
                    None
                }
            })
            .sum::<u32>();

    Ok(format!("{}", sum))
}

pub fn part2(input: &str) -> PuzzleResult {
    let digits = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| format!("Unable to parse char to digit: {c}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let list = List(digits);
    let sum = list
        .iter()
        .enumerate()
        .filter_map(|(idx, &d)| {
            if d == list.halfway_digit(idx) {
                Some(d)
            } else {
                None
            }
        })
        .sum::<u32>();

    Ok(format!("{}", sum))
}

struct List(Vec<u32>);

impl List {
    /// Get the digit halfway around the circular list from the given index
    fn halfway_digit(&self, idx: usize) -> u32 {
        let h_idx = (idx + self.len() / 2) % self.len();
        self.0[h_idx]
    }

    /// Get an iterator over the underlying [`Vec`]
    fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)];

        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{}", expected));
        }
    }
}
