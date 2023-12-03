use std::ops::Index;

use itertools::Itertools;

use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let schematic = Schematic::from(input);
    let sum: u32 = schematic.part_numbers().iter().sum();

    Ok(format!("{sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let schematic = Schematic::from(input);
    let sum: u32 = schematic.gears().iter().map(|g| g.ratio()).sum();

    Ok(format!("{sum}"))
}

/// An engine schematic, consisting of a grid of numbers,
/// symbols, and `.` characters.
#[derive(Debug)]
struct Schematic(Vec<Vec<char>>);

impl Schematic {
    /// Fetch all the part numbers in the schematic.
    ///
    /// A number adjacent to a symbol (including diagonally)
    /// in the schematic is a part number.
    fn part_numbers(&self) -> Vec<u32> {
        self.symbols()
            .iter()
            .map(|s| self.adjacent(s.coords()))
            .flatten()
            .filter_map(|c| self.number_at(&c))
            .unique()
            .map(|sn| sn.val())
            .collect()
    }

    /// Get all the [`Symbol`]s in the schematic
    fn symbols(&self) -> Vec<Symbol> {
        let num_rows = self.0.len();
        (&self.0)
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, &c)| {
                if c.is_ascii_alphanumeric() || c == '.' {
                    None
                } else {
                    let row = idx / num_rows;
                    let col = idx % num_rows;

                    Some(Symbol::new(c, row, col))
                }
            })
            .collect()
    }

    /// Get all adjacent (including diagonals) [`Coordinates`]
    /// in the schematic relative to the given `Coordinates`
    fn adjacent(&self, loc: &Coordinates) -> Vec<Coordinates> {
        let deltas = [
            (0, 1),   // right
            (1, 1),   // down right
            (1, 0),   // down
            (1, -1),  // down left
            (0, -1),  // left
            (-1, -1), // up left
            (-1, 0),  // up
            (-1, 1),  // up right
        ];

        deltas
            .iter()
            .filter_map(|(dr, dc)| {
                let (r, c) = loc.0;
                let row = r as i32 + dr;
                let col = c as i32 + dc;

                // Make sure the coordinate is in bounds
                if row < 0 || col < 0 {
                    None
                } else if row >= self.0.len() as i32 || col >= self.0[0].len() as i32 {
                    None
                } else {
                    Some(Coordinates::new(row as usize, col as usize))
                }
            })
            .collect()
    }

    /// Get the number with a digit at the given [`Coordinates`].
    ///
    /// If no such number can be constructed, return [`None`]
    fn number_at(&self, loc: &Coordinates) -> Option<SchemaNum> {
        if !self[loc].is_numeric() {
            return None;
        }

        let digits_to_left = {
            let (row, mut col) = loc.0;

            while col > 0 && self[(row, col - 1)].is_numeric() {
                col -= 1;
            }

            &self.0[row][col..loc.col()]
        };

        let digits_to_right = {
            let (row, mut col) = loc.0;

            while col + 1 < self.0[0].len() && self[(row, col + 1)].is_numeric() {
                col += 1;
            }

            &self.0[row][loc.col() + 1..=col]
        };

        let digits = {
            let mut digits = Vec::new();

            for d in digits_to_left {
                digits.push(d.to_digit(10).unwrap());
            }

            digits.push(self[loc].to_digit(10).unwrap());

            for d in digits_to_right {
                digits.push(d.to_digit(10).unwrap());
            }

            digits
        };

        let mut num = 0;
        for (digit, tens) in digits
            .iter()
            .zip(digits.iter().enumerate().map(|(idx, _)| idx).rev())
        {
            let mut d = *digit;
            for _ in 0..tens {
                d *= 10;
            }
            num += d;
        }

        let start = Coordinates::new(loc.row(), loc.col() - digits_to_left.len());
        let stop = Coordinates::new(loc.row(), loc.col() + digits_to_right.len());
        Some(SchemaNum::new(num, start, stop))
    }

    /// Fetch all the gears in the schematic.
    ///
    /// A gear is a `*` [`Symbol`] adjacent to exactly two part numbers.
    fn gears(&self) -> Vec<Gear> {
        self.symbols()
            .iter()
            .filter_map(|s| {
                if s.val() == '*' {
                    let adj_nums = self
                        .adjacent(s.coords())
                        .iter()
                        .filter_map(|c| self.number_at(c))
                        .unique()
                        .collect_vec();

                    if adj_nums.len() == 2 {
                        Some(Gear::new(adj_nums[0], adj_nums[1]))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        Self(s.lines().map(|l| l.trim().chars().collect()).collect())
    }
}

impl Index<&Coordinates> for Schematic {
    type Output = char;

    fn index(&self, index: &Coordinates) -> &Self::Output {
        &self.0[index.row()][index.col()]
    }
}

impl Index<(usize, usize)> for Schematic {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.0[row][col]
    }
}

/// Helper type that represents a symbol in the [`Schematic`]
/// as well as its coordinates in the `Schematic`
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Symbol((char, Coordinates));

impl Symbol {
    fn new(val: char, row: usize, col: usize) -> Self {
        Self((val, Coordinates::new(row, col)))
    }

    fn val(&self) -> char {
        self.0 .0
    }

    fn coords(&self) -> &Coordinates {
        &self.0 .1
    }
}

/// Helper type that represents a set of coordinates in the
/// [`Schematic`]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Coordinates((usize, usize));

impl Coordinates {
    fn new(row: usize, col: usize) -> Self {
        Self((row, col))
    }

    fn row(&self) -> usize {
        self.0 .0
    }

    fn col(&self) -> usize {
        self.0 .1
    }
}

impl From<&(usize, usize)> for Coordinates {
    fn from(tuple: &(usize, usize)) -> Self {
        Self((tuple.0, tuple.1))
    }
}

/// Helper type to represent a number in the [`Schematic`]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct SchemaNum {
    val: u32,
    start: Coordinates,
    stop: Coordinates,
}

impl SchemaNum {
    fn new(val: u32, start: Coordinates, stop: Coordinates) -> Self {
        Self { val, start, stop }
    }

    fn val(&self) -> u32 {
        self.val
    }
}

/// Helper type to represent a special type of [`Symbol`] in
/// the [`Schematic`]
#[derive(Debug)]
struct Gear((SchemaNum, SchemaNum));

impl Gear {
    fn new(num1: SchemaNum, num2: SchemaNum) -> Self {
        Self((num1, num2))
    }

    fn ratio(&self) -> u32 {
        self.0 .0.val() * self.0 .1.val()
    }

    /// Return the two [`SchemaNum`] components of this gear in ascending order
    fn nums_ordered(&self) -> (SchemaNum, SchemaNum) {
        if self.0 .0 > self.0 .1 {
            (self.0 .1, self.0 .0)
        } else {
            (self.0 .0, self.0 .1)
        }
    }
}

impl std::cmp::PartialEq for Gear {
    /// Two gears are equal if they have equal [`SchemaNum`] components,
    /// irrespective of order
    fn eq(&self, other: &Self) -> bool {
        self.nums_ordered().eq(&other.nums_ordered())
    }

    /// Two gears are not equal if they do not have equal [`SchemaNum`]
    /// components
    fn ne(&self, other: &Self) -> bool {
        self.nums_ordered().ne(&other.nums_ordered())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    mod schematic {
        use super::*;

        #[test]
        fn part_numbers() {
            let mut expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
            expected.sort();

            let schematic = Schematic::from(INPUT);
            let mut actual = schematic.part_numbers();
            actual.sort();

            assert_eq!(actual, expected, "actual != expected!");
        }

        #[test]
        fn symbols() {
            let mut expected = vec![
                Symbol::new('*', 1, 3),
                Symbol::new('#', 3, 6),
                Symbol::new('*', 4, 3),
                Symbol::new('+', 5, 5),
                Symbol::new('$', 8, 3),
                Symbol::new('*', 8, 5),
            ];
            expected.sort();

            let mut actual = Schematic::from(INPUT).symbols();
            actual.sort();

            assert_eq!(actual, expected, "actual != expected!");
        }

        #[test]
        fn adjacent() {
            let schematic = Schematic::from(INPUT);
            let cases = vec![
                (
                    "top left corner",
                    Coordinates::new(0, 0),
                    vec![(0, 1), (1, 0), (1, 1)],
                ),
                (
                    "top right corner",
                    Coordinates::new(0, 9),
                    vec![(0, 8), (1, 8), (1, 9)],
                ),
                (
                    "bottom left corner",
                    Coordinates::new(9, 0),
                    vec![(8, 0), (8, 1), (9, 1)],
                ),
                (
                    "bottom right corner",
                    Coordinates::new(9, 9),
                    vec![(9, 8), (8, 8), (8, 9)],
                ),
                (
                    "somewhere in the middle",
                    Coordinates::new(5, 4),
                    vec![
                        (5, 5),
                        (6, 5),
                        (6, 4),
                        (6, 3),
                        (5, 3),
                        (4, 3),
                        (4, 4),
                        (4, 5),
                    ],
                ),
            ];

            for (comment, input, expected) in cases {
                let expected = {
                    let mut expected: Vec<Coordinates> =
                        expected.iter().map(|tuple| tuple.into()).collect();
                    expected.sort();
                    expected
                };

                let actual = {
                    let mut actual = schematic.adjacent(&input);
                    actual.sort();
                    actual
                };

                assert_eq!(actual, expected, "actual != expected in case: {comment}");
            }
        }

        #[test]
        fn get_num() {
            let schematic = Schematic::from(INPUT);
            let cases = vec![
                (Coordinates::new(0, 0), 467),
                (Coordinates::new(2, 3), 35),
                (Coordinates::new(6, 3), 592),
            ];

            for (input, expected) in cases {
                assert_eq!(
                    schematic.number_at(&input).unwrap().val,
                    expected,
                    "actual != expected"
                );
            }

            assert_eq!(
                schematic.number_at(&Coordinates::new(1, 0)),
                None,
                "actual != expected"
            );
        }

        #[test]
        fn gears() {
            let schematic = Schematic::from(INPUT);
            let expected = vec![
                Gear::new(
                    SchemaNum::new(467, Coordinates::new(0, 0), Coordinates::new(0, 2)),
                    SchemaNum::new(35, Coordinates::new(2, 2), Coordinates::new(2, 3)),
                ),
                Gear::new(
                    SchemaNum::new(755, Coordinates::new(7, 6), Coordinates::new(7, 8)),
                    SchemaNum::new(598, Coordinates::new(9, 5), Coordinates::new(9, 7)),
                ),
            ];

            assert_eq!(schematic.gears(), expected, "actual != expected");
        }
    }

    #[test]
    fn p1() {
        let expected = 4361;
        assert_eq!(
            part1(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }

    #[test]
    fn p2() {
        let expected = 467835;
        assert_eq!(
            part2(INPUT).unwrap(),
            format!("{expected}"),
            "actual != expected"
        );
    }
}
