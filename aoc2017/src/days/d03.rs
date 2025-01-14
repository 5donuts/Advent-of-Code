use utils::PuzzleResult;

pub fn part1(input: &str) -> PuzzleResult {
    let target_addr = input
        .lines()
        .map(|l| l.parse::<usize>())
        .next()
        .ok_or_else(|| format!("No addr in: {input}"))??;

    let ring = Ring::containing(target_addr);

    let dist = if ring.corners().contains(&target_addr) {
        ring.num() + 1
    } else {
        // Get the "side" of the ring on which this address can be found.
        let (c1, c2) = ring
            .corners()
            .windows(2)
            .filter_map(|c| {
                if c[0] < target_addr && target_addr < c[1] {
                    Some((c[0] as i32, c[1] as i32))
                } else {
                    None
                }
            })
            .next()
            // If the sliding window across the counter-clockwise ordered corners doesn't yield the
            // side containing the target address, it must be on the side between the first and last
            // corners which is not examined by the sliding window.
            .or_else(|| {
                let corners = ring.corners();
                Some((corners[3] as i32, corners[0] as i32))
            })
            .unwrap();

        // Compute the distance between the address and the origin using `n + 1 - d` where `d` is
        // the difference between the nearest corner and the target address.
        let d = (c1 - target_addr as i32)
            .abs()
            .min((c2 - target_addr as i32).abs()) as usize;
        ring.num() + 1 - d
    };

    Ok(format!("{dist}"))
}

pub fn part2(input: &str) -> PuzzleResult {
    let input: usize = input.parse()?;

    todo!("Not implemented")
}

/// Represent a "ring" of addresses in the spiral memory grid.
///
/// ## Part 1
///
/// The first step to solving the puzzle is to find the "ring" in the spiral memory grid in which
/// the target address can be found.
///
/// This is the layout of the grid in question:
///
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23  24  25 ...
///
/// Notice that if you start at address 1 and move down & right, then down & right again, each
/// of these "corner" addresses is a perfect square.
///
/// For every odd n, we can generate the range of addresses in the ring as follows:
///
/// ring start: n^2 + 1
/// ring end: (n + 2)^2
///
/// This yields:
///
/// Ring 1: (2, 9)
/// Ring 2: (5, 16) --> this is NOT a valid ring
/// Ring 3: (10, 25) --> this IS a valid ring
/// Ring 4: invalid
/// Ring 5: (26, 49)
/// ...
///
/// With this in mind, we can quickly narrow in on the ring that contains the target address.
///
/// For example, `sqrt(12) ≈ 3.46`, rounded down is 3; hence 12 should be found on Ring 3 (10, 25).
///
/// Then, once we know the ring that contains the given address, it's easy to determine the
/// distance to the origin. There are exactly three options for each address in a ring:
/// 1. The address is a corner, so the distance is `n + 1`.
/// 2. The address is along an axis to the origin (e.g., 2, 11, 28), so the distance is `n - 2`.
/// 3. The address is between a corner and an axis, so the distance is equal to `n + 1 - d` where
///    `d` is the difference between the address and the closest corner.
///
/// In practice, options 2 and 3 are identical because when the address is on an axis with the
/// origin, `n + 1 - d = n - 2`.
#[derive(Debug)]
struct Ring(usize);

impl Ring {
    /// Rings can only be indexed by odd numbers, so only odd indices are accepted here.
    ///
    /// This is due to the ring definition functions I'm using:
    ///
    /// ring start: n^2 + 1
    /// ring end: (n + 2)^2
    ///
    /// This yields:
    ///
    /// Ring 1: (2, 9)
    /// Ring 2: (5, 16) --> this is NOT a valid ring
    /// Ring 3: (10, 25) --> this IS a valid ring
    /// Ring 4: (17, 36) --> another invalid ring
    /// Ring 5: (26, 49)
    /// ...
    fn new(n: usize) -> Option<Self> {
        if n % 2 == 0 {
            None
        } else {
            Some(Self(n))
        }
    }

    /// Get the ring that contains the given address.
    ///
    /// This is determined by `sqrt(addr)` and rounding down to the nearest whole odd number. That
    /// is the `n` value to use with [`Ring::new`] to get the ring containing the specified
    /// address.
    fn containing(addr: usize) -> Self {
        let mut sqrt = addr.isqrt();
        if sqrt % 2 == 0 {
            sqrt -= 1
        }

        Self::new(sqrt).unwrap()
    }

    /// Get the "number" of this ring (i.e., 1, 3, 5, ..., n)
    fn num(&self) -> usize {
        self.0
    }

    /// Get the memory address at the start of this ring.
    fn start(&self) -> usize {
        self.num().pow(2) + 1
    }

    /// Get the memory address at the end of this ring.
    fn end(&self) -> usize {
        (self.num() + 2).pow(2)
    }

    /// Get the four corner addresses of this ring in ascending, counter-clockwise order starting
    /// with the "top-right" corner.
    fn corners(&self) -> Vec<usize> {
        // Step by `n + 1` because the number of elements on each "side" of the ring is `n + 2`,
        // meaning there are `n + 2 - 1` steps between those elements.
        let step = self.num() + 1;
        let start = self.end();

        (0..4)
            .into_iter()
            .map(|i| start - (i * step))
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Get the steps from the address back to the origin
    #[test]
    fn p1() {
        let cases = vec![("12", "3"), ("23", "2"), ("1024", "31")];

        for (input, expected) in cases {
            assert_eq!(part1(input).unwrap(), expected);
        }
    }

    #[test]
    fn p2() {
        todo!("Not implemented");
    }

    mod ring {
        use super::*;

        // Ensure rings have the expected start/end points
        #[test]
        fn bounds() {
            let cases = vec![
                (1, Some(2), Some(9)),
                (2, None, None),
                (3, Some(10), Some(25)),
                (4, None, None),
                (5, Some(26), Some(49)),
                (6, None, None),
                (7, Some(50), Some(81)),
                (8, None, None),
                (9, Some(82), Some(121)),
            ];

            for (input, expected_start, expected_end) in cases {
                assert_eq!(
                    Ring::new(input).and_then(|r| Some(r.start())),
                    expected_start,
                    "Ring {input} start"
                );
                assert_eq!(
                    Ring::new(input).and_then(|r| Some(r.end())),
                    expected_end,
                    "Ring {input} end"
                );
            }
        }

        // Ensure we can select the correct rings to contain arbitrary addresses
        #[test]
        fn containing() {
            let cases = vec![(12, 3), (23, 3), (1024, 31)];

            for (input, expected) in cases {
                assert_eq!(Ring::containing(input).0, expected)
            }
        }

        // Ensure we can get the expected corners for rings
        #[test]
        fn corners() {
            let cases = vec![(3, vec![3, 5, 7, 9]), (5, vec![13, 17, 21, 25])];

            for (input, expected) in cases {
                let ring = Ring::new(input).unwrap();
                assert_eq!(ring.corners(), expected);
            }
        }
    }
}
