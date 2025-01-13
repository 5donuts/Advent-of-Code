use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let possible_triangles = Triangle::parse_p1(input)?
        .iter()
        .filter(|t| t.valid())
        .count();

    Ok(format!("{possible_triangles}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let possible_triangles = Triangle::parse_p2(input)?
        .iter()
        .filter(|t| t.valid())
        .count();

    Ok(format!("{possible_triangles}"))
}

#[derive(Debug)]
struct Triangle((u32, u32, u32));

impl Triangle {
    /// Given the side lengths for this triangle, determine if it's actually a valid triangle
    fn valid(&self) -> bool {
        let sides = self.0;

        // Need to check all side combinations to ensure validity
        sides.0 + sides.1 > sides.2 && sides.1 + sides.2 > sides.0 && sides.2 + sides.0 > sides.1
    }

    /// Given the puzzle input, parse the input according to the rules specified for Part 1
    fn parse_p1(input: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        input
            .lines()
            .map(|l| {
                let mut s = l.split_whitespace();
                let s1 = s
                    .next()
                    .ok_or_else(|| format!("No first number in line: {l}"))?
                    .parse::<u32>()?;
                let s2 = s
                    .next()
                    .ok_or_else(|| format!("No second number in line: {l}"))?
                    .parse::<u32>()?;
                let s3 = s
                    .next()
                    .ok_or_else(|| format!("No third first number in line: {l}"))?
                    .parse::<u32>()?;

                Ok(Self((s1, s2, s3)))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Given the puzzle input, parse the input according to the rules specified for Part 2
    fn parse_p2(input: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let col1 = input
            .lines()
            .map(|l| {
                let s = l
                    .split_whitespace()
                    .next()
                    .ok_or_else(|| format!("No first number in line: {l}"))?;
                s.parse::<u32>().map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        let col2 = input
            .lines()
            .map(|l| {
                let s = l
                    .split_whitespace()
                    .skip(1)
                    .next()
                    .ok_or_else(|| format!("No second number in line: {l}"))?;
                s.parse::<u32>().map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        let col3 = input
            .lines()
            .map(|l| {
                let s = l
                    .split_whitespace()
                    .skip(2)
                    .next()
                    .ok_or_else(|| format!("No third number in line: {l}"))?;
                s.parse::<u32>().map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(col1
            .chunks(3)
            .chain(col2.chunks(3))
            .chain(col3.chunks(3))
            .map(|sides| Self((sides[0], sides[1], sides[2])))
            .collect())
    }
}

#[cfg(test)]
// Allow comparing the Triangle to a tuple describing side lengths for `assert_eq` in the tests
// below
impl std::cmp::PartialEq<(u32, u32, u32)> for Triangle {
    fn eq(&self, other: &(u32, u32, u32)) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_p1() {
        let input = r#"5 10 25
1 2 3
3 4 5"#;

        let expected = vec![(5, 10, 25), (1, 2, 3), (3, 4, 5)];

        assert_eq!(Triangle::parse_p1(input).unwrap(), expected);
    }

    // Test input processing for triangles in part2
    #[test]
    fn parse_p2() {
        let input = r#"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603"#;

        let expected = vec![
            (101, 102, 103),
            (201, 202, 203),
            (301, 302, 303),
            (401, 402, 403),
            (501, 502, 503),
            (601, 602, 603),
        ];

        assert_eq!(Triangle::parse_p2(input).unwrap(), expected);
    }
}
