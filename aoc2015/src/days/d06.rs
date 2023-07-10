use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.try_into().unwrap()).collect();
    let mut g: Grid<p1::Light> = Grid::new();

    for i in instructions {
        match i.action() {
            Action::TurnOn => g.turn_on(i.start(), i.stop()),
            Action::TurnOff => g.turn_off(i.start(), i.stop()),
            Action::Toggle => g.toggle(i.start(), i.stop()),
        }
    }

    Ok(format!(
        "{}",
        g.lights().filter(|&&l| l == p1::Light::On).count()
    ))
}

pub fn part2(input: &str) -> PuzzleResult {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.try_into().unwrap()).collect();
    let mut g: Grid<p2::Light> = Grid::new();

    for i in instructions {
        match i.action() {
            Action::TurnOn => g.turn_on(i.start(), i.stop()),
            Action::TurnOff => g.turn_off(i.start(), i.stop()),
            Action::Toggle => g.toggle(i.start(), i.stop()),
        }
    }

    Ok(format!(
        "{}",
        g.lights().map(|l| l.brightness()).sum::<usize>()
    ))
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Instruction((Action, (usize, usize), (usize, usize)));

impl Instruction {
    fn action(&self) -> Action {
        self.0 .0
    }

    fn start(&self) -> (usize, usize) {
        self.0 .1
    }

    fn stop(&self) -> (usize, usize) {
        self.0 .2
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.split(" ");

        let action = {
            let mut action_str = split.next().ok_or("Could not get action string")?;

            if action_str == "turn" {
                action_str = split
                    .next()
                    .ok_or("Could not get second part of action string")?;
            }

            match action_str {
                "on" => Ok(Action::TurnOn),
                "off" => Ok(Action::TurnOff),
                "toggle" => Ok(Action::Toggle),
                _ => Err(format!("Invalid action string: {}", action_str)),
            }
        }?;

        let start = {
            let start_str = split.next().ok_or("Could not get start coords")?;

            let mut start_split = start_str.split(",");
            let x = start_split.next().ok_or("No start x coord")?.parse()?;
            let y = start_split.next().ok_or("No start y coord")?.parse()?;

            (x, y)
        };

        // Drop the 'through' portion of the input
        split.next().ok_or("Missing 'through'")?;

        let stop = {
            let stop_str = split.next().ok_or("Could not get start coords")?;

            let mut stop_split = stop_str.split(",");
            let x = stop_split.next().ok_or("No start x coord")?.parse()?;
            let y = stop_split.next().ok_or("No start y coord")?.parse()?;

            (x, y)
        };

        Ok(Self((action, start, stop)))
    }
}

/// Represent a single light in the [Grid] for either [part1] or [part2]
trait Light {
    /// Create a new (turned off) light
    fn new() -> Self;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

/// The side length of a square grid of [Light]s.
const GRID_SIZE: usize = 1000;

struct Grid<T>(Box<Vec<Vec<T>>>)
where
    T: Light + Clone + Copy;

impl<T> Grid<T>
where
    T: Light + Clone + Copy,
{
    fn new() -> Self {
        Self(Box::new(vec![vec![T::new(); GRID_SIZE]; GRID_SIZE]))
    }

    fn turn_on(&mut self, start: (usize, usize), stop: (usize, usize)) {
        let (x1, y1) = start;
        let (x2, y2) = stop;

        for x in x1..=x2 {
            for y in y1..=y2 {
                self[x][y].turn_on();
            }
        }
    }

    fn turn_off(&mut self, start: (usize, usize), stop: (usize, usize)) {
        let (x1, y1) = start;
        let (x2, y2) = stop;

        for x in x1..=x2 {
            for y in y1..=y2 {
                self[x][y].turn_off();
            }
        }
    }

    fn toggle(&mut self, start: (usize, usize), stop: (usize, usize)) {
        let (x1, y1) = start;
        let (x2, y2) = stop;

        for x in x1..=x2 {
            for y in y1..=y2 {
                self[x][y].toggle();
            }
        }
    }

    fn lights(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flat_map(|col| col.iter())
    }
}

impl<T> std::ops::Index<usize> for Grid<T>
where
    T: Light + Clone + Copy,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index).as_slice()
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T>
where
    T: Light + Clone + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index).as_mut_slice()
    }
}

// Only used by a couple tests
#[cfg(test)]
impl<T> std::ops::Index<std::ops::RangeTo<usize>> for Grid<T>
where
    T: Light + Clone + Copy,
{
    type Output = [Vec<T>];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

// Only used by a couple tests
#[cfg(test)]
impl<T> std::ops::Index<std::ops::RangeFrom<usize>> for Grid<T>
where
    T: Light + Clone + Copy,
{
    type Output = [Vec<T>];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

mod p1 {
    /// A single [Light](super::Light) in [part1](super::part1).
    ///
    /// These lights are either On or Off.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Light {
        On,
        Off,
    }

    impl super::Light for Light {
        fn new() -> Self {
            Self::Off
        }

        fn turn_on(&mut self) {
            *self = Light::On;
        }

        fn turn_off(&mut self) {
            *self = Light::Off;
        }

        fn toggle(&mut self) {
            *self = match self {
                Light::On => Light::Off,
                Light::Off => Light::On,
            };
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn turn_on() {
            // Turn on all the lights
            let mut g: Grid<super::Light> = Grid::new();
            g.turn_on((0, 0), (999, 999));
            assert_eq!(
                g.lights().filter(|&&l| l == super::Light::On).count(),
                1_000_000,
                "All lights should be on"
            );
        }

        #[test]
        fn turn_off() {
            // Start with a fully on grid for testing
            let mut g: Grid<super::Light> = Grid::new();
            g.turn_on((0, 0), (999, 999));
            assert_eq!(
                g.lights().filter(|&&l| l == super::Light::On).count(),
                1_000_000,
                "All lights should be on"
            );

            // Turn off only the middle 4 lights
            g.turn_off((499, 499), (500, 500));
            for (row, col) in vec![(499, 499), (499, 500), (500, 499), (500, 500)] {
                assert_eq!(
                    g[row][col],
                    super::Light::Off,
                    "Middle four lights should be off"
                );
            }
        }

        #[test]
        fn toggle() {
            // Light every other column of lights in the grid for testing
            let mut g: Grid<super::Light> = Grid::new();
            for row in 0..1000 {
                for col in (0..1000).into_iter().filter(|i| i % 2 == 0) {
                    g[row][col] = super::Light::On;
                }
            }
            assert_eq!(
                g.lights().filter(|&&l| l == super::Light::On).count(),
                500_000,
                "Half the lights should be on"
            );

            g.toggle((0, 0), (999, 999));
            assert_eq!(
                g.lights().filter(|&&l| l == super::Light::On).count(),
                500_000,
                "Half the lights should be on after toggle"
            );

            for row in 0..1000 {
                for col in 0..1000 {
                    if col % 2 == 0 {
                        assert!(
                            g[row][col] == super::Light::Off,
                            "Light at ({}, {}) should be off",
                            row,
                            col
                        );
                    } else {
                        assert!(
                            g[row][col] == super::Light::On,
                            "Light at ({}, {}) should be on",
                            row,
                            col
                        );
                    }
                }
            }
        }
    }
}

mod p2 {
    /// A single [Light](super::Light) in [part2](super::part2).
    ///
    /// These lights have a brightness rather than an on/off state.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Light(usize);

    impl Light {
        pub fn brightness(&self) -> usize {
            self.0
        }
    }

    impl super::Light for Light {
        fn new() -> Self {
            Self(0)
        }

        fn turn_on(&mut self) {
            *self = Self(self.0 + 1);
        }

        fn turn_off(&mut self) {
            *self = if self.0 > 0 {
                Self(self.0 - 1)
            } else {
                Self(0)
            };
        }

        fn toggle(&mut self) {
            *self = Self(self.0 + 2);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn turn_on() {
            let mut g: Grid<super::Light> = Grid::new();
            g.turn_on((0, 0), (0, 0));
            assert!(g.lights().map(|l| l.brightness()).sum::<usize>() == 1);
        }

        #[test]
        fn toggle() {
            let mut g: Grid<super::Light> = Grid::new();
            g.toggle((0, 0), (999, 999));
            assert!(g.lights().map(|l| l.brightness()).sum::<usize>() == 2_000_000);
        }

        #[test]
        fn brightness() {
            let mut light = super::Light::new();
            assert_eq!(light.brightness(), 0);

            for i in 0..100 {
                light.turn_on();
                assert_eq!(light.brightness(), i + 1);
            }

            for i in (1..=100).rev() {
                light.turn_off();
                assert_eq!(light.brightness(), i - 1);
            }

            for i in 0..100 {
                light.toggle();
                assert_eq!(light.brightness(), (i + 1) * 2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_try_from() {
        let cases = vec![
            (
                "turn on 0,0 through 999,999",
                Instruction((Action::TurnOn, (0, 0), (999, 999))),
            ),
            (
                "toggle 0,0 through 999,0",
                Instruction((Action::Toggle, (0, 0), (999, 0))),
            ),
            (
                "turn off 499,499 through 500,500",
                Instruction((Action::TurnOff, (499, 499), (500, 500))),
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(Instruction::try_from(input).unwrap(), expected);
        }
    }
}
