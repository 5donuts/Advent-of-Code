use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let presents = input
        .lines()
        .map(|l| {
            let mut parts = l.split("x");
            let l = parts
                .next()
                .ok_or_else(|| format!("No length for: {l}"))?
                .parse::<u32>()?;
            let w = parts
                .next()
                .ok_or_else(|| format!("No width for: {l}"))?
                .parse::<u32>()?;
            let h = parts
                .next()
                .ok_or_else(|| format!("No height for: {l}"))?
                .parse::<u32>()?;

            Ok(Present::from((l, w, h)))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    let total_area: u32 = presents.iter().map(|p| p.surf_area() + p.slack()).sum();

    Ok(format!("{}", total_area))
}

pub fn part2(input: &str) -> PuzzleResult {
    let presents = input
        .lines()
        .map(|l| {
            let mut parts = l.split("x");
            let l = parts
                .next()
                .ok_or_else(|| format!("No length for: {l}"))?
                .parse::<u32>()?;
            let w = parts
                .next()
                .ok_or_else(|| format!("No width for: {l}"))?
                .parse::<u32>()?;
            let h = parts
                .next()
                .ok_or_else(|| format!("No height for: {l}"))?
                .parse::<u32>()?;

            Ok(Present::from((l, w, h)))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    let total_ribbon: u32 = presents.iter().map(|p| p.ribbon_len() + p.volume()).sum();

    Ok(format!("{}", total_ribbon))
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn surf_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn slack(&self) -> u32 {
        let side_areas = vec![self.l * self.w, self.l * self.h, self.w * self.h];
        *side_areas.iter().min().unwrap()
    }

    fn ribbon_len(&self) -> u32 {
        let perimeters = vec![
            2 * self.l + 2 * self.w,
            2 * self.l + 2 * self.h,
            2 * self.w + 2 * self.h,
        ];
        *perimeters.iter().min().unwrap()
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

impl From<(u32, u32, u32)> for Present {
    fn from(val: (u32, u32, u32)) -> Self {
        let (l, w, h) = val;
        Self { l, w, h }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let cases = vec![("2x3x4", 58), ("1x1x10", 43)];
        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), format!("{}", expected));
        }
    }

    #[test]
    fn p2() {
        let cases = vec![("2x3x4", 34), ("1x1x10", 14)];
        for (input, expected) in cases {
            assert_eq!(part2(input).unwrap(), format!("{}", expected));
        }
    }
}
