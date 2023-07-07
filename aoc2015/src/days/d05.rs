use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let nice_cnt = input.lines().filter(|s| p1::is_nice(s)).count();

    Ok(format!("{}", nice_cnt))
}

pub fn part2(input: &str) -> PuzzleResult {
    let nice_cnt = input.lines().filter(|s| p2::is_nice(s)).count();

    Ok(format!("{}", nice_cnt))
}

mod p1 {
    use super::char_windows;

    pub fn is_nice(s: &str) -> bool {
        has_three_vowels(s) && has_double_letter(s) && no_bad_strings(s)
    }

    fn has_three_vowels(s: &str) -> bool {
        s.chars()
            .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
            .count()
            >= 3
    }

    fn has_double_letter(s: &str) -> bool {
        char_windows(s, 2)
            .filter(|w| {
                let mut chars = w.chars();
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();
                a == b
            })
            .count()
            > 0
    }

    fn no_bad_strings(s: &str) -> bool {
        char_windows(s, 2)
            .filter(|&w| w == "ab" || w == "cd" || w == "pq" || w == "xy")
            .count()
            == 0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn triple_vowels() {
            for s in vec!["aei", "xazegov", "aeiouaeiouaeiou"] {
                assert!(has_three_vowels(s), "String '{}' has three vowels", s);
            }
        }

        #[test]
        fn double_letters() {
            for s in vec!["xx", "abcdde", "aabbccdd"] {
                assert!(has_double_letter(s), "String '{}' has double letters", s);
            }
        }

        #[test]
        fn nice() {
            for s in vec!["ugknbfddgicrmopn", "aaa"] {
                assert!(is_nice(s), "String '{}' is actually nice", s);
            }

            for s in vec!["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"] {
                assert!(!is_nice(s), "String '{}' is actually naughty", s);
            }
        }
    }
}

mod p2 {
    use super::char_windows;
    use std::cmp::{Ord, Ordering, PartialOrd};
    use std::collections::{HashMap, HashSet};

    pub fn is_nice(s: &str) -> bool {
        has_double_pairs_no_overlap(s) && has_interrupted_double_letter(s)
    }

    fn has_double_pairs_no_overlap(s: &str) -> bool {
        let pairs = sort_pairs(s);

        // Get all of the duplicate pairs
        let mut key_counts = HashMap::new();
        for k in pairs.iter().map(|p| p.key()) {
            let count = key_counts.get(k).unwrap_or(&0);
            key_counts.insert(k, count + 1);
        }
        let duplicate_keys: HashSet<_> = key_counts
            .iter()
            .filter_map(|(k, v)| if v > &1 { Some(k) } else { None })
            .collect();
        let duplicate_pairs: Vec<_> = pairs
            .iter()
            .filter(|p| duplicate_keys.contains(&p.key()))
            .collect();

        // Get all combinations of the duplicate pairs, get them into a single iterator of combinations, then filter
        // to only combinations that don't overlap
        get_pair_combinations(&duplicate_pairs)
            .iter()
            .filter(|(_, qs)| qs.len() > 0)
            .flat_map(|(p, qs)| qs.iter().map(move |q| (p, q)))
            .filter(|(p, q)| {
                let p = p.idx();
                let q = q.idx();
                let (max, min) = if p > q { (p, q) } else { (q, p) };
                // A distance of 1 indicates the original windows for these two pairs of chars in the source string
                // were adjacent. If they were adjacent, they overlap (b/c the windows themselves overlap).
                max - min > 1
            })
            .count()
            > 0
    }

    fn has_interrupted_double_letter(s: &str) -> bool {
        char_windows(s, 3)
            .filter(|w| {
                let mut chars = w.chars();
                let a = chars.next().unwrap();
                chars.next().unwrap(); // this letter doesn't actually matter
                let c = chars.next().unwrap();
                a == c
            })
            .count()
            > 0
    }

    /// Get all combinations of the given (sorted) [`Pair`]s that have matching keys.
    /// Note: this function does _not_ consider mirror duplicates to be unique.
    /// I.e., if you have some pair `(a, b)` then `(b, a)` is _not_ considered a separate pair.
    fn get_pair_combinations<'a>(pairs: &'a [&Pair]) -> Vec<(&'a Pair<'a>, Vec<&'a Pair<'a>>)> {
        let mut combinations = Vec::new();

        for (idx, &p) in pairs.iter().enumerate() {
            // Get all the remaining pairs with the same key as the current one.
            // This is why the pairs need to be sorted for this function.
            let qs = pairs[(idx + 1)..]
                .iter()
                .filter_map(|&q| if q.key() == p.key() { Some(q) } else { None })
                .collect();

            combinations.push((p, qs));
        }

        combinations
    }

    /// Get a sorted list of all the [`char`] pairs in the given string
    ///
    /// # Returns
    /// A list of `(index, key)` pairs where `key` is the pair of characters in the string and `index`
    /// is the index of the [Window](std::slice::Windows) of that pair (from [char_windows]).
    fn sort_pairs(s: &str) -> Vec<Pair> {
        // Sort the pairs, keeping track of their relative location in the string
        let mut pairs: Vec<_> = char_windows(s, 2)
            .enumerate()
            .map(|(idx, s)| Pair((idx, s)))
            .collect();
        pairs.sort_unstable();

        pairs
    }

    /// Wrapper type to represent a single pair from the input string.
    /// A pair consists of two letters, plus its index relative to the other pairs.
    #[derive(Debug, PartialEq, Eq)]
    struct Pair<'a>((usize, &'a str));

    impl<'a> Pair<'a> {
        fn key(&self) -> &str {
            self.0 .1
        }

        fn idx(&self) -> usize {
            self.0 .0
        }
    }

    impl<'a> PartialOrd for Pair<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<'a> Ord for Pair<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            // Order primarily based on the underlying strings, only use the index
            // when the strings are equal
            let order = self.key().cmp(&other.key());
            match order {
                Ordering::Equal => self.idx().cmp(&other.idx()),
                _ => order,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn double_pairs_no_overlap() {
            for s in vec![
                "xyxy",
                "aabcdefgaa",
                "qjhvhtzxzqqjkmpb",
                "xxyxx",
                "uurcxstgmygtbstg",
                "xxxxyx", // Had to rewrite all of part 2 b/c of this case :(
            ] {
                assert!(
                    has_double_pairs_no_overlap(s),
                    "String '{}' actually has a pair that appears twice with no overlap",
                    s
                );
            }

            assert!(
                !has_double_pairs_no_overlap("aaa"),
                "String 'aaa' actually has an overlapping pair"
            );
        }

        #[test]
        fn interrupted_double_letter() {
            for s in vec![
                "xyx",
                "abcdefeghi",
                "aaa",
                "qjhvhtzxzqqjkmpb",
                "xxyxx",
                "ieodomkazucvgmuy",
            ] {
                assert!(
                    has_interrupted_double_letter(s),
                    "String '{}' has an interrupted double letter",
                    s
                );
            }
        }

        #[test]
        fn nice() {
            for s in vec!["qjhvhtzxzqqjkmpb", "xxyxx"] {
                assert!(is_nice(s), "String '{}' is actually nice", s);
            }

            for s in vec!["uurcxstgmygtbstg", "ieodomkazucvgmuy"] {
                assert!(!is_nice(s), "String '{}' is actually naughty", s);
            }
        }

        #[test]
        fn pair_sorting() {
            let cases = vec![
                (
                    "xyxy",
                    vec![Pair((0, "xy")), Pair((2, "xy")), Pair((1, "yx"))],
                ),
                (
                    "xxyxx",
                    vec![
                        Pair((0, "xx")),
                        Pair((3, "xx")),
                        Pair((1, "xy")),
                        Pair((2, "yx")),
                    ],
                ),
                (
                    "xxxxyx",
                    vec![
                        Pair((0, "xx")),
                        Pair((1, "xx")),
                        Pair((2, "xx")),
                        Pair((3, "xy")),
                        Pair((4, "yx")),
                    ],
                ),
            ];
            for (input, expected) in cases {
                assert_eq!(sort_pairs(input), expected);
            }
        }

        #[test]
        fn pair_combos() {
            let cases = vec![
                (
                    "xyxy",
                    vec![
                        (&Pair((0, "xy")), vec![&Pair((2, "xy"))]),
                        (&Pair((1, "yx")), vec![]),
                        (&Pair((2, "xy")), vec![]),
                    ],
                ),
                (
                    "xxyxx",
                    vec![
                        (&Pair((0, "xx")), vec![&Pair((3, "xx"))]),
                        (&Pair((3, "xx")), vec![]),
                        (&Pair((1, "xy")), vec![]),
                        (&Pair((2, "yx")), vec![]),
                    ],
                ),
                (
                    "xxxxyx",
                    vec![
                        (&Pair((0, "xx")), vec![&Pair((1, "xx")), &Pair((2, "xx"))]),
                        (&Pair((1, "xx")), vec![&Pair((2, "xx"))]),
                        (&Pair((2, "xx")), vec![]),
                        (&Pair((3, "xy")), vec![]),
                        (&Pair((4, "yx")), vec![]),
                    ],
                ),
            ];
            for (input, mut expected) in cases {
                let pairs = sort_pairs(input);
                expected.sort_unstable();
                let pairs: Vec<_> = pairs.iter().map(|p| p).collect(); // &[Pair] -> &[&Pair]

                let mut actual = get_pair_combinations(&pairs);
                actual.sort_unstable();

                assert_eq!(actual, expected, "Failed to evaluate pairs for {}", input);
            }
            // todo!()
        }
    }
}

/// This creates sliding windows across a [String] without any allocations.
/// For more details, see:
/// * https://stackoverflow.com/a/51261570/3646065
/// * [std::slice::Windows]
fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows() {
        let s = "abcdef";
        let expected = vec!["ab", "bc", "cd", "de", "ef"];
        assert_eq!(char_windows(s, 2).count(), expected.len());
        for (idx, w) in char_windows(s, 2).enumerate() {
            assert_eq!(w, expected[idx]);
        }
    }
}
