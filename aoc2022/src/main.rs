mod days;
use utils::Day;

fn main() {
    let days = vec![
        Day {
            num: 1,
            p1: days::d01::part1,
            p2: days::d01::part2,
            input: "aoc2022/input/1",
        },
        Day {
            num: 2,
            p1: days::d02::part1,
            p2: days::d02::part2,
            input: "aoc2022/input/2",
        },
        Day {
            num: 3,
            p1: days::d03::part1,
            p2: days::d03::part2,
            input: "aoc2022/input/3",
        },
        Day {
            num: 4,
            p1: days::d04::part1,
            p2: days::d04::part2,
            input: "aoc2022/input/4",
        },
    ];

    for day in days {
        day.run().unwrap();
    }
}
