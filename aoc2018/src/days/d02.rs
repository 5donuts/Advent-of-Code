use utils::PuzzleResult;

use std::collections::HashMap;

pub fn part1(input: &str) -> PuzzleResult {
    let box_ids = input
        .lines()
        .map(|l| {
            let mut map: HashMap<char, usize> = HashMap::default();
            for c in l.chars() {
                if let Some(count) = map.get(&c) {
                    map.insert(c, count + 1);
                } else {
                    map.insert(c, 1);
                }
            }
            map
        })
        .collect::<Vec<_>>();

    let two_letters = box_ids
        .iter()
        .filter(|map| map.iter().filter(|(_, &count)| count == 2).count() > 0)
        .count();

    let three_letters = box_ids
        .iter()
        .filter(|map| map.iter().filter(|(_, &count)| count == 3).count() > 0)
        .count();

    Ok(format!("{}", two_letters * three_letters))
}

pub fn part2(input: &str) -> PuzzleResult {
    let box_ids = input.lines().collect::<Vec<_>>();

    // Get the two IDs that have exactly one character different at the same position
    let shared_letters = box_ids
        .iter()
        .enumerate()
        .filter_map(|(idx, id)| {
            // Ensure there is at least one ID to compare against.
            if idx + 1 < box_ids.len() {
                // Compare the chars of this ID with the chars of each remaining ID.
                box_ids[idx + 1..]
                    .iter()
                    .filter_map(move |other| {
                        // Build the list of letters shared between the two IDs (considering
                        // position in the ID).
                        let shared_letters = id
                            .chars()
                            .zip(other.chars())
                            .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                            .collect::<String>();

                        // If there is a difference of only one letter, this is our puzzle
                        // solution.
                        if shared_letters.len() == id.len() - 1 {
                            Some(shared_letters)
                        } else {
                            None
                        }
                    })
                    .next()
            } else {
                None
            }
        })
        .next()
        .ok_or("No matching box IDs found!")?;

    Ok(shared_letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;

        assert_eq!(part1(input).unwrap(), "12");
    }

    #[test]
    fn p2() {
        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;

        assert_eq!(part2(input).unwrap(), "fgij");
    }
}
