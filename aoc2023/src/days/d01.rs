use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let sum: u32 = input.lines().map(|l| calibration_value(l)).sum();
    Ok(format!("{sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let sum: u32 = input
        .lines()
        .map(|l| calibration_value(&unspell_digits(l)))
        .sum();
    Ok(format!("{sum}"))
}

/// Fetch the first and last digits of a line to construct the calibration value
/// represented by that line.
fn calibration_value(line: &str) -> u32 {
    let first = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let last = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .rev()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    (first * 10) + last
}

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn all() -> &'static [Digit] {
        use Digit::*;

        &[One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
    }

    fn len(&self) -> usize {
        self.chars().len()
    }

    fn chars(&self) -> &'static [char] {
        use Digit::*;

        match self {
            One => &['o', 'n', 'e'],
            Two => &['t', 'w', 'o'],
            Three => &['t', 'h', 'r', 'e', 'e'],
            Four => &['f', 'o', 'u', 'r'],
            Five => &['f', 'i', 'v', 'e'],
            Six => &['s', 'i', 'x'],
            Seven => &['s', 'e', 'v', 'e', 'n'],
            Eight => &['e', 'i', 'g', 'h', 't'],
            Nine => &['n', 'i', 'n', 'e'],
        }
    }

    fn value(&self) -> char {
        use Digit::*;

        match self {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
        }
    }
}

/// Convert the "spelled-out" digits in a line (e.g., `four` for `4`) to the
/// corresponding ASCII digits.
///
/// If two digits overlap, only the one encountered first when reading left
/// to right is converted to an ASCII digit.
fn unspell_digits(line: &str) -> String {
    let digits = Digit::all();
    let mut res = String::new();

    // Advance a cursor across the characters in the line.
    let chars: Vec<_> = line.chars().collect();
    let mut cursor = 0;
    let mut checking_overlap = false;
    while cursor < chars.len() {
        let mut next_char = None;
        let remaining_chars = chars.len() - cursor;

        // If there are too few remaining letters or the cursor is already
        // pointing at an ASCII digit, append the digit at the cursor to
        // the return string & advance the cursor. Don't do any of this
        // short-circuiting if we are checking for overlaps.
        if !checking_overlap {
            if remaining_chars < 3 {
                next_char = Some(chars[cursor]);
            } else if chars[cursor].is_ascii_digit() {
                next_char = Some(chars[cursor]);
            }

            if let Some(c) = next_char {
                res.push(c);
                cursor = cursor + 1;
                continue;
            }
        }

        // If there is a digit word that starts at the cursor, append the corresponding
        // ASCII digit to the return string, then advance the cursor to the final character
        // in the digit word. This allows for overlapping digit words to be detected.
        for digit in digits {
            if digit.len() > remaining_chars {
                continue;
            }

            let word = &chars[cursor..(cursor + digit.len())];

            if word.starts_with(digit.chars()) {
                next_char = Some(digit.value());
                cursor = cursor + digit.len() - 1;
                checking_overlap = true;

                break; // out of the inner loop
            }
        }

        // Otherwise, append the character at the cursor to the return string and move
        // the cursor forward by one character so long as we are not checking for an
        // overlapping digit word. In that case, we do not want to add the current
        // character.
        if next_char.is_none() {
            if !checking_overlap {
                next_char = Some(chars[cursor]);
            }

            checking_overlap = false;
            cursor = cursor + 1;
        }

        if let Some(c) = next_char {
            res.push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(part1(input).unwrap(), format!("{}", 142));
    }

    #[test]
    fn p2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(part2(input).unwrap(), format!("{}", 281));
    }

    #[test]
    fn calibration_val() {
        let cases = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (input, expected) in cases {
            assert_eq!(calibration_value(input), expected);
        }
    }

    #[test]
    fn str_to_digit() {
        // The examples for part 2 of this problem left a critical piece of information
        // up to interpretation.
        //
        // Specifically, would a line like `zoneight234` become `z1ight234` or `z18234`?
        //
        // I assumed the former, initially, and got the wrong answer to the problem.
        //
        // So, I must assume the latter is the correct interpretation. There are cases
        // in my input where there _would_ be a different digit at the end of the line
        // depending on the interpretation, hence my initial incorrect answer.
        let cases = vec![
            ("two1nine", "219"),
            ("eightwothree", "823"),
            ("abcone2threexyz", "abc123xyz"),
            ("xtwone3four", "x2134"),
            ("4nineeightseven2", "49872"),
            ("zoneight234", "z18234"),
            ("7pqrstsixteen", "7pqrst6teen"),
        ];

        for (input, expected) in cases {
            assert_eq!(unspell_digits(input), expected);
        }
    }
}
