use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let directions: Vec<(char, i32)> = input
        .trim()
        .split(", ")
        .map(|s| {
            let direction = s.chars().next().unwrap();
            let distance = s[1..].parse().unwrap();

            (direction, distance)
        })
        .collect();

    let mut position = (0, 0);
    let mut ordinal = Ordinal::North;
    for direction in directions {
        ordinal = match direction.0 {
            'L' => ordinal.left(),
            'R' => ordinal.right(),
            _ => panic!("Invalid input!"),
        };

        use Ordinal::*;
        let distance = direction.1;
        position = match ordinal {
            North => (position.0, position.1 + distance),
            South => (position.0, position.1 - distance),
            East => (position.0 + distance, position.1),
            West => (position.0 - distance, position.1),
        };
    }

    let distance = position.0.abs() + position.1.abs();
    Ok(format!("{}", distance))
}

pub fn part2(input: &str) -> PuzzleResult {
    let directions: Vec<(char, i32)> = input
        .trim()
        .split(", ")
        .map(|s| {
            let direction = s.chars().next().unwrap();
            let distance = s[1..].parse().unwrap();

            (direction, distance)
        })
        .collect();

    // Build the list of all visited points (including points between the endpoints defined by the
    // instructions).
    let mut visited = vec![(0, 0)];
    let mut ordinal = Ordinal::North;
    for direction in directions {
        ordinal = match direction.0 {
            'L' => ordinal.left(),
            'R' => ordinal.right(),
            _ => panic!("Invalid input!"),
        };

        use Ordinal::*;
        let distance = direction.1;
        let prev_pos = visited.last().unwrap();
        let next_pos = match ordinal {
            North => (prev_pos.0, prev_pos.1 + distance),
            South => (prev_pos.0, prev_pos.1 - distance),
            East => (prev_pos.0 + distance, prev_pos.1),
            West => (prev_pos.0 - distance, prev_pos.1),
        };
        let line = LineSegment::new(*prev_pos, next_pos);
        let mut line_points = line.points();
        visited.append(&mut line_points);
    }

    // Find the first point in the list that has previously been visited
    let visited = visited; // mark as immutable for multiple concurrent refs
    let position = visited
        .iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if visited[..idx].contains(p) {
                Some(p)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    let distance = position.0.abs() + position.1.abs();
    Ok(format!("{}", distance))
}

enum Ordinal {
    North,
    South,
    East,
    West,
}

impl Ordinal {
    /// Get the ordinal 90 degrees to the left of this one
    fn left(&self) -> Ordinal {
        use Ordinal::*;

        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    /// Get the ordinal 90 degrees to the right of this one
    fn right(&self) -> Ordinal {
        self.left().left().left()
    }
}

struct LineSegment {
    p1: (i32, i32),
    p2: (i32, i32),
}

impl LineSegment {
    fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        Self { p1, p2 }
    }

    /// Return the points between the two endpoints that define this line segment (including `p2`
    /// but excluding `p1` since `p1` should already be in the list of visited points).
    fn points(&self) -> Vec<(i32, i32)> {
        let (x1, y1) = self.p1;
        let (x2, y2) = self.p2;

        // Case 1: moving in the +x direction
        if x1 < x2 {
            let dist = x2 - x1;
            (1..=dist).map(|x| (x1 + x, y1)).collect()
        }
        // Case 2: moving in the -x direction
        else if x1 > x2 {
            let dist = x1 - x2;
            (1..=dist).map(|x| (x1 - x, y1)).collect()
        }
        // Case 3: moving in the +y direction
        else if y1 < y2 {
            let dist = y2 - y1;
            (1..=dist).map(|y| (x1, y1 + y)).collect()
        }
        // Case 4: moving in the -y direction
        else {
            let dist = y1 - y2;
            (1..=dist).map(|y| (x1, y1 - y)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];

        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let input = "R8, R4, R4, R8";
        let expected = 4;

        assert_eq!(part2(input).unwrap(), format!("{}", expected));
    }

    mod line_segment {
        use super::*;

        #[test]
        fn points() {
            let cases = vec![
                (
                    LineSegment::new((0, 0), (8, 0)),
                    vec![
                        (1, 0),
                        (2, 0),
                        (3, 0),
                        (4, 0),
                        (5, 0),
                        (6, 0),
                        (7, 0),
                        (8, 0),
                    ],
                ),
                (
                    LineSegment::new((8, 0), (8, -4)),
                    vec![(8, -1), (8, -2), (8, -3), (8, -4)],
                ),
                (
                    LineSegment::new((8, -4), (4, -4)),
                    vec![(7, -4), (6, -4), (5, -4), (4, -4)],
                ),
                (
                    LineSegment::new((4, -4), (4, 4)),
                    vec![
                        (4, -3),
                        (4, -2),
                        (4, -1),
                        (4, 0),
                        (4, 1),
                        (4, 2),
                        (4, 3),
                        (4, 4),
                    ],
                ),
            ];

            for (input, expected) in cases {
                assert_eq!(input.points(), expected, "actual != expected");
            }
        }
    }
}
