use itertools::Itertools;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let input = input.trim();
    let sop_mkr = Datastream(input).start_of_packet_marker().unwrap();
    Ok(format!("{}", sop_mkr))
}

pub fn part2(input: &str) -> PuzzleResult {
    let input = input.trim();
    let som_mkr = Datastream(input).start_of_message_marker().unwrap();
    Ok(format!("{}", som_mkr))
}

struct Datastream<'a>(&'a str);

impl<'a> Datastream<'a> {
    fn start_of_packet_marker(&self) -> Result<usize, String> {
        let stream = self.0;

        if let Some(end_idx) = (0..stream.len())
            .into_iter()
            .filter_map(|idx| {
                let s = &stream[idx..idx + 4];
                if s.chars().unique().count() == 4 {
                    Some(idx + 4) // idx after the start of packet marker
                } else {
                    None
                }
            })
            .next()
        {
            Ok(end_idx)
        } else {
            Err(format!(
                "No start-of-packet marker found in stream '{}'",
                stream
            ))
        }
    }

    fn start_of_message_marker(&self) -> Result<usize, String> {
        let stream = self.0;

        if let Some(end_idx) = (0..stream.len())
            .into_iter()
            .filter_map(|idx| {
                let s = &stream[idx..idx + 14];
                if s.chars().unique().count() == 14 {
                    Some(idx + 14) // idx after the start of message marker
                } else {
                    None
                }
            })
            .next()
        {
            Ok(end_idx)
        } else {
            Err(format!(
                "No start-of-message marker found in stream '{}'",
                stream
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [&'static str; 5] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn p1() {
        let expected = vec![7, 5, 6, 10, 11];
        assert!(
            expected.len() == INPUT.len(),
            "Bad test; input & expected values diff len"
        );
        for (idx, s) in INPUT.iter().enumerate() {
            assert_eq!(
                Datastream(s).start_of_packet_marker().unwrap(),
                expected[idx]
            );
        }
    }

    #[test]
    fn p2() {
        let expected = vec![19, 23, 23, 29, 26];
        assert!(
            expected.len() == INPUT.len(),
            "Bad test; input & expected values diff len"
        );
        for (idx, s) in INPUT.iter().enumerate() {
            assert_eq!(
                Datastream(s).start_of_message_marker().unwrap(),
                expected[idx]
            );
        }
    }
}
