use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let input: Vec<_> = input
        .lines()
        .into_iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    for i in 0..input.len() {
        for j in i..input.len() {
            let i = input[i];
            let j = input[j];

            if i + j == 2020 {
                return Ok(format!("{}", i * j));
            }
        }
    }
    Err("Answer not found".into())
}

pub fn part2(input: &str) -> PuzzleResult {
    let input: Vec<_> = input
        .lines()
        .into_iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    for i in 0..input.len() {
        for j in i..input.len() {
            for k in j..input.len() {
                let i = input[i];
                let j = input[j];
                let k = input[k];

                if i + j + k == 2020 {
                    return Ok(format!("{}", i * j * k));
                }
            }
        }
    }
    Err("Answer not found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "514579");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "241861950");
    }
}
