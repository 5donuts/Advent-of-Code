use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let mut digits = Vec::new();
    let mut keypad = Keypad::new();
    for mut instructions in input.lines().map(|l| l.chars()) {
        while let Some(c) = instructions.next() {
            match c {
                'U' => keypad = keypad.up(),
                'D' => keypad = keypad.down(),
                'L' => keypad = keypad.left(),
                'R' => keypad = keypad.right(),
                _ => panic!("Invalid instruction: {c}"),
            }
        }
        digits.push(keypad.digit());
    }

    let code = digits
        .iter()
        .map(|d| format!("{d}"))
        .collect::<Vec<_>>()
        .join("");
    Ok(code)
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut digits = Vec::new();
    let mut keypad = CursedKeypad::new();
    for mut instructions in input.lines().map(|l| l.chars()) {
        while let Some(c) = instructions.next() {
            match c {
                'U' => keypad = keypad.up(),
                'D' => keypad = keypad.down(),
                'L' => keypad = keypad.left(),
                'R' => keypad = keypad.right(),
                _ => panic!("Invalid instruction: {c}"),
            }
        }
        digits.push(keypad.digit().unwrap());
    }

    let code = digits
        .iter()
        .map(|d| format!("{d}"))
        .collect::<Vec<_>>()
        .join("");
    Ok(code)
}

struct Keypad(u32);

impl Keypad {
    fn new() -> Self {
        Self(5)
    }

    fn digit(&self) -> u32 {
        self.0
    }

    /// Move up one row on the keypad (if able)
    fn up(self) -> Self {
        if self.0 > 3 {
            Self(self.0 - 3)
        } else {
            self
        }
    }

    /// Move down one row on the keypad (if able)
    fn down(self) -> Self {
        if self.0 < 7 {
            Self(self.0 + 3)
        } else {
            self
        }
    }

    /// Move left one col on the keypad (if able)
    fn left(self) -> Self {
        if self.0 % 3 != 1 {
            Self(self.0 - 1)
        } else {
            self
        }
    }

    /// move right one rol on the keypad (if able)
    fn right(self) -> Self {
        if self.0 % 3 != 0 {
            Self(self.0 + 1)
        } else {
            self
        }
    }
}

/// Grid for the [CursedKeypad] to use
const CURSED_GRID: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

struct CursedKeypad {
    x: usize,
    y: usize,
}

impl CursedKeypad {
    fn new() -> Self {
        Self { x: 0, y: 2 }
    }

    fn digit(&self) -> Option<char> {
        CURSED_GRID[self.y][self.x]
    }

    /// Move up one row on the keypad (if able)
    fn up(self) -> Self {
        if self.y == 0 {
            self
        } else if let Some(_) = CURSED_GRID[self.y - 1][self.x] {
            Self {
                x: self.x,
                y: self.y - 1,
            }
        } else {
            self
        }
    }

    /// Move down one row on the keypad (if able)
    fn down(self) -> Self {
        if self.y == 4 {
            self
        } else if let Some(_) = CURSED_GRID[self.y + 1][self.x] {
            Self {
                x: self.x,
                y: self.y + 1,
            }
        } else {
            self
        }
    }

    /// Move left one col on the keypad (if able)
    fn left(self) -> Self {
        if self.x == 0 {
            self
        } else if let Some(_) = CURSED_GRID[self.y][self.x - 1] {
            Self {
                x: self.x - 1,
                y: self.y,
            }
        } else {
            self
        }
    }

    /// move right one rol on the keypad (if able)
    fn right(self) -> Self {
        if self.x == 4 {
            self
        } else if let Some(_) = CURSED_GRID[self.y][self.x + 1] {
            Self {
                x: self.x + 1,
                y: self.y,
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"ULL
RRDDD
LURDL
UUUUD"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "1985");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "5DB3");
    }
}
