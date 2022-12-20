use itertools::Itertools;
use std::collections::HashMap;
use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let total_priorities: u32 = input
        .lines()
        .into_iter()
        .map(|l| {
            let rs = Rucksack::from(l);
            let dup_prios: u32 = rs.items_in_both().iter().map(|&c| priority(c)).sum();
            dup_prios
        })
        .sum();

    Ok(format!("{}", total_priorities))
}

pub fn part2(input: &str) -> PuzzleResult {
    let rucksacks: Vec<_> = input
        .lines()
        .into_iter()
        .map(|l| Rucksack::from(l))
        .collect();
    let groups: Vec<_> = rucksacks
        .chunks(3)
        .map(|g| {
            let g: [Rucksack<'_>; 3] = g.try_into().unwrap();
            Group(g)
        })
        .collect();
    let badge_priorities: u32 = groups.iter().map(|g| priority(g.find_badge())).sum();

    Ok(format!("{}", badge_priorities))
}

struct Group<'a>([Rucksack<'a>; 3]);

impl<'a> Group<'a> {
    fn find_badge(&self) -> char {
        // store the number of rucksacks in which you can find each item
        let mut items: HashMap<char, usize> = HashMap::new();

        // get the unique items in each rucksack & add them to the map
        for rs in &self.0 {
            for item in rs.c1.chars().chain(rs.c2.chars()).unique() {
                if let Some(count) = items.get(&item) {
                    items.insert(item, count + 1);
                } else {
                    items.insert(item, 1);
                }
            }
        }

        // find the badge item
        items
            .iter()
            .filter_map(|(&item, &count)| if count == 3 { Some(item) } else { None })
            .next()
            .unwrap()
    }
}

#[derive(Clone, Copy)]
struct Rucksack<'a> {
    c1: &'a str,
    c2: &'a str,
}

impl<'a> Rucksack<'a> {
    fn items_in_both(&self) -> Vec<char> {
        self.c1
            .chars()
            .into_iter()
            .filter(|&c| self.c2.contains(c))
            .unique()
            .collect()
    }
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        let c_sz = s.len() / 2;

        let c1 = &s[..c_sz];
        let c2 = &s[c_sz..];

        Self { c1, c2 }
    }
}

fn priority(c: char) -> u32 {
    let ascii = c as u32;

    // 91 is ASCII code for '[' (char just after 'Z')
    if ascii < 91 {
        // uppercase chars
        ascii - 38 // this maps ASCII code 65 ('A') to priority value 27
    } else {
        // lowercase chars
        ascii - 96 // this maps ASCII code 97 ('a') to priority value 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn p1() {
        assert_eq!(part1(&INPUT).unwrap(), "157".to_string());
    }

    #[test]
    fn p2() {
        assert_eq!(part2(&INPUT).unwrap(), "70".to_string());
    }

    #[test]
    fn check_compartments() {
        let expected = vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
        ];

        let actual = &INPUT.lines().collect::<Vec<&str>>()[..3];
        for (idx, &compartments) in expected.iter().enumerate() {
            let (c1, c2) = compartments;
            let rs = Rucksack::from(actual[idx]);
            assert_eq!(rs.c1, c1);
            assert_eq!(rs.c2, c2);
        }
    }

    #[test]
    fn check_priority() {
        let expected = vec![
            ('a', 1),
            ('b', 2),
            // [...]
            ('y', 25),
            ('z', 26),
            ('A', 27),
            ('B', 28),
            // [...]
            ('Y', 51),
            ('Z', 52),
        ];

        for (item, prio) in expected {
            let actual_prio = priority(item);
            assert_eq!(
                actual_prio, prio,
                "Incorrect priority ({}) for item '{}'; expected {}",
                actual_prio, item, prio
            );
        }
    }

    #[test]
    fn check_items_in_both() {
        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];

        for (idx, line) in INPUT.lines().enumerate() {
            let rs = Rucksack::from(line);
            assert_eq!(rs.items_in_both(), vec![expected[idx]]);
        }
    }

    #[test]
    fn check_badges() {
        let expected = vec!['r', 'Z'];

        let rucksacks: Vec<_> = INPUT
            .lines()
            .into_iter()
            .map(|l| Rucksack::from(l))
            .collect();
        let groups: Vec<_> = rucksacks
            .chunks(3)
            .map(|g| {
                let g: [Rucksack<'_>; 3] = g.try_into().unwrap();
                Group(g)
            })
            .collect();

        for (idx, g) in groups.iter().enumerate() {
            assert_eq!(g.find_badge(), expected[idx]);
        }
    }
}
