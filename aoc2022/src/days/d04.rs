use itertools::Itertools;
use std::collections::HashSet;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let fully_contained_pairs = input
        .lines()
        .map(|l| {
            // convert each line to two sets containing all of the section numbers
            // in the specified range
            let mut sets = l.split(",").map(|s| {
                // get the start/end of the range
                let mut s = s.split("-");
                let start = s.next().unwrap().parse().unwrap();
                let end = s.next().unwrap().parse().unwrap();

                // convert to a set
                let set: HashSet<u32> = (start..=end).collect();

                set
            });

            // return the sets as a tuple
            sets.next_tuple().unwrap()
        })
        .filter(|(a, b)| {
            let a_diff_b = a.difference(b).count();
            let b_diff_a = b.difference(a).count();

            // determine if either set is fully contained by the other
            a_diff_b == 0 || b_diff_a == 0
        })
        .count();

    Ok(format!("{}", fully_contained_pairs))
}

pub fn part2(input: &str) -> PuzzleResult {
    let intersecting_pairs = input
        .lines()
        .map(|l| {
            // convert each line to two sets containing all of the section numbers
            // in the specified range
            let mut sets = l.split(",").map(|s| {
                // get the start/end of the range
                let mut s = s.split("-");
                let start = s.next().unwrap().parse().unwrap();
                let end = s.next().unwrap().parse().unwrap();

                // convert to a set
                let set: HashSet<u32> = (start..=end).collect();

                set
            });

            // return the sets as a tuple
            sets.next_tuple().unwrap()
        })
        .filter(|(a, b)| {
            // determine if there is an overlap in the sets
            a.intersection(b).count() > 0
        })
        .count();

    Ok(format!("{}", intersecting_pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn p1() {
        assert_eq!(part1(&INPUT).unwrap(), "2");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(&INPUT).unwrap(), "4");
    }
}
