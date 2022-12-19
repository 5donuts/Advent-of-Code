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
    ];

    for day in days {
        day.run().unwrap();
    }
}
