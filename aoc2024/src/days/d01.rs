use utils::PuzzleResult;

use std::collections::HashMap;

pub fn part1(input: &str) -> PuzzleResult {
    let (mut list1, mut list2) = prepare_lists(input)?;

    list1.sort_unstable();
    list2.sort_unstable();

    let distances_sum = list1
        .iter()
        .zip(list2)
        .map(|(l1, l2)| (l2 - l1).abs())
        .sum::<i32>();

    Ok(format!("{distances_sum}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let (list1, list2) = prepare_lists(input)?;

    // Build the list of counts in the second list
    let mut list2_counts: HashMap<i32, u32> = HashMap::default();
    for item in list2 {
        if let Some(count) = list2_counts.get(&item) {
            list2_counts.insert(item, count + 1);
        } else {
            list2_counts.insert(item, 1);
        }
    }

    let similarity_score = list1
        .iter()
        .filter_map(|&item| {
            if let Some(&count) = list2_counts.get(&item) {
                Some(item as u32 * count)
            } else {
                None
            }
        })
        .sum::<u32>();
    Ok(format!("{similarity_score}"))
}

/// Parse the input into the two lists of numbers
fn prepare_lists(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    Ok(input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let l = it.next().ok_or("No left number")?.parse::<i32>()?;
            let r = it.next().ok_or("No right number")?.parse::<i32>()?;

            Ok((l, r))
        })
        // TODO: figure out how to remove the `collect` to process the `Result`
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .into_iter()
        .unzip())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT).unwrap(), "11");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT).unwrap(), "31");
    }
}
