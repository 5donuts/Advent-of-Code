use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let max_calories: u32 = input
        .replace("\r", "")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .ok_or("Could not fetch max calories".to_string())?;

    Ok(format!("{}", max_calories))
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut elf_calories: Vec<u32> = input
        .replace("\r", "")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    elf_calories.sort_by(|a, b| b.cmp(a)); // reverse sort (i.e., descending)

    let top_three_cals: u32 = elf_calories[..3].iter().sum();
    Ok(format!("{}", top_three_cals))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "24000");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "45000");
    }
}
